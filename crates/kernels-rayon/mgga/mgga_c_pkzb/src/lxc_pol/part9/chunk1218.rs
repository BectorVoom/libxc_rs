//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1218/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1218(t1096: f64, t17388: f64, t17616: f64, t21220: f64, t21223: f64, t21225: f64, t21226: f64, t21229: f64, t21233: f64, t21236: f64, t21239: f64, t21251: f64, t21255: f64, t21257: f64, t2801: f64, t2820: f64, t5830: f64, t5831: f64, t5883: f64, t704: f64, t723: f64, t7486: f64) -> f64 {
    let t21258 = -t21220 - t21223 - t21225 + 0.17544670867903938621e1_f64 * t21226 * t723 + 3.0_f64 * t21229 * t704 - t21233 - t21236 - t21239 - 24.0_f64 * t5830 * t1096 * t5831 - 6.0_f64 * t7486 * t5883 - 6.0_f64 * t17388 * t2801 + 0.96491876992155210402e2_f64 * t17616 * t2820 + t21251 - t21255 - t21257;
    t21258
}
