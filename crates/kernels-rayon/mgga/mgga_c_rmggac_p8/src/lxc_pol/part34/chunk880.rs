//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 880/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk880(t14077: f64, t15290: f64, t7282: f64, t12200: f64, t15313: f64, t15227: f64, t70207: f64, t1971: f64, t495: f64, t7230: f64, t875: f64, t8936: f64) -> (f64, f64, f64, f64) {
    let t75736 = t7282 * t14077 * t15290;
    let t75739 = t12200 * t14077 * t15313;
    let t75748 = t70207 * t15227;
    let t75756 = t7230 * t1971 * t875 * t8936 * t495;
    (t75736, t75739, t75748, t75756)
}
