//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 847/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk847(t41906: f64, t34478: f64, t544: f64, t9287: f64, t12793: f64, t1441: f64, t40192: f64, t40196: f64, t41860: f64, t41863: f64, t41867: f64, t41871: f64, t41874: f64, t41876: f64, t41880: f64, t41885: f64, t41889: f64, t41891: f64, t41893: f64, t41897: f64, t41900: f64, t41904: f64, t41905: f64, t536: f64, t590: f64) -> f64 {
    let t41907 = 0.15337170381568299871e1_f64 * t41906;
    let t41909 = t544 * t34478 * t9287;
    let t41911 = -0.38342925953920749676e0_f64 * t40192 + 0.85206502119823888169e-1_f64 * t40196 + 0.35750489951850426669e0_f64 * t536 * t41860 + 0.23005755572352449806e2_f64 * t41863 + 0.23005755572352449806e2_f64 * t41867 + 0.23005755572352449806e2_f64 * t41871 + t41874 + 0.13803453343411469884e2_f64 * t41876 - 0.42900587942220512004e1_f64 * t41880 + t41885 - t41889 - 0.38342925953920749676e0_f64 * t41891 + t41893 + 0.51123901271894332902e0_f64 * t1441 * t12793 * t590 + 0.23833659967900284447e0_f64 * t41897 + 0.38342925953920749676e0_f64 * t41900 + t41904 - t41905 + t41907 + 0.29792074959875355558e-1_f64 * t41909;
    t41911
}
