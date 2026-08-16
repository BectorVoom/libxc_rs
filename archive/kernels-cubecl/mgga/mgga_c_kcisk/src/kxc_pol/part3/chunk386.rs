//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 386/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk386<F: Float>(t776: F, t794: F, t772: F, t1689: F, t41: F, t1758: F, t525: F, t642: F, t773: F, t79: F, t781: F) -> (F, F, F, F, F, F) {
    let t777 = t776 < -F::cast_from(0.66725e-1_f64);
    let t1992 = t794 * t794;
    let t1993 = F::cast_from(1.0_f64) / t1992;
    let t1994 = t772 * t1993;
    let t1995 = t1689 * t41;
    let t2003 = piecewise3::<F>(t777, F::cast_from(0.0_f64), F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t525 * t1995 * t642 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t525 * t773 * t1758);
    let t2004 = t79 * t2003;
    let t2005 = t2004 * t781;
    (t1992, t1993, t1994, t1995, t2004, t2005)
}
