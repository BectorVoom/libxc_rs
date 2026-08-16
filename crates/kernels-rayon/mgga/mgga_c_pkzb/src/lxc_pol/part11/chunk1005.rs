//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1005/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1005(t11008: f64, t11052: f64, t158: f64, t1143: f64, t3675: f64, t6000: f64, t2964: f64, t3694: f64, t10942: f64, t5728: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11053 = t11008 + t11052;
    let t11054 = t11053 * t158;
    let t11063 = t3675 * t1143;
    let t11064 = t6000 * t11063;
    let t11067 = t2964 * t3694;
    let t11070 = t10942 * t5728;
    (t11053, t11054, t11063, t11064, t11067, t11070)
}
