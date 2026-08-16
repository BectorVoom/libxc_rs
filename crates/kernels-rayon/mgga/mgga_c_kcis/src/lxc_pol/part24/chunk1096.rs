//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1096/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1096(t1749: f64, t1774: f64, t303: f64, t6614: f64, t7726: f64, t26679: f64, t6272: f64, t4947: f64, t1709: f64, t27778: f64, t26686: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28966 = t1749 * t1774;
    let t28967 = t303 * t28966;
    let t28973 = t7726 * t6614;
    let t28974 = t303 * t28973;
    let t28983 = t26679 * t6272;
    let t28984 = t4947 * t28983;
    let t28987 = t27778 * t1709;
    let t28988 = t26686 * t28987;
    (t28966, t28967, t28973, t28974, t28983, t28984, t28987, t28988)
}
