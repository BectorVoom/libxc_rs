//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 624/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk624<F: Float>(t776: F, t2028: F, t791: F, t1992: F, t794: F, t772: F, t41: F, t4794: F, t1758: F, t1995: F, t4973: F, t4977: F, t525: F, t642: F, t773: F) -> (F, F, F, F, F, F, F, F) {
    let t777 = t776 < -F::cast_from(0.66725e-1_f64);
    let t5437 = t2028 * t2028;
    let t5438 = t791 * t791;
    let t5439 = F::cast_from(1.0_f64) / t5438;
    let t5440 = t5437 * t5439;
    let t5444 = F::cast_from(1.0_f64) / t1992 / t794;
    let t5445 = t772 * t5444;
    let t5449 = t4794 * t41;
    let t5463 = piecewise3::<F>(t777, F::cast_from(0.0_f64), F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t525 * t5449 * t642 - F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t525 * t1995 * t1758 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t525 * t773 * t4973 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t525 * t773 * t4977);
    (t5437, t5438, t5439, t5440, t5444, t5445, t5449, t5463)
}
