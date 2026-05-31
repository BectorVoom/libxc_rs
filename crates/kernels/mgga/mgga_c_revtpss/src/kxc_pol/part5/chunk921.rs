//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 921/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk921<F: Float>(t30: F, t33: F, t189: F, t6800: F, t512: F, t1344: F, t3874: F, t5824: F, t6785: F, t1348: F, t3881: F, t6416: F, t6792: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t6801 = t6800 * t189;
    let t6802 = t512 * t6801;
    let t6808 = piecewise3::<F>(t31, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3874 * t6785 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1344 * t5824);
    let t6814 = piecewise3::<F>(t34, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3881 * t6792 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1348 * t6416);
    let t6816 = t6808 / F::cast_from(2.0_f64) + t6814 / F::cast_from(2.0_f64);
    (t6801, t6802, t6816)
}
