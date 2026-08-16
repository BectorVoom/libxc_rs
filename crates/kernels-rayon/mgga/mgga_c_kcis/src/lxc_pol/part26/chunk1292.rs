//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1292/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1292(t1464: f64, t1497: f64, t60761: f64, t7923: f64, t16622: f64, t28504: f64, t491: f64, t28338: f64, t98470: f64, t1928: f64, t4122: f64, t98409: f64) -> (f64, f64, f64, f64, f64) {
    let t102102 = t1464 * t7923 * t60761 * t1497;
    let t102106 = t1464 * t16622 * t491 * t28504;
    let t102109 = t1464 * t98470 * t28338;
    let t102115 = t1464 * t4122 * t1928 * t28504;
    let t102118 = t1464 * t98409 * t28338;
    (t102102, t102106, t102109, t102115, t102118)
}
