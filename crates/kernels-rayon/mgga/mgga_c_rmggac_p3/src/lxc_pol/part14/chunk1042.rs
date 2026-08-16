//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1042/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1042(t1971: f64, t236: f64, t5704: f64, t7365: f64, t35331: f64, t5700: f64, t36772: f64, t9147: f64, t615: f64, t7230: f64, t839: f64, t880: f64) -> (f64, f64, f64, f64) {
    let t41690 = t7365 * t1971 * t236 * t5704;
    let t41694 = t35331 * t1971 * t236 * t5700;
    let t41696 = t36772 * t9147;
    let t41701 = t7230 * t1971 * t880 * t615 * t839;
    (t41690, t41694, t41696, t41701)
}
