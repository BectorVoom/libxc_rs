//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1112/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1112(t2496: f64, t4251: f64, t952: f64, t10914: f64, t957: f64, t238: f64, t4261: f64, t801: f64, t1392: f64, t3470: f64, t242: f64, t4265: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10927 = t2496 * t4251;
    let t10928 = t10927 * t952;
    let t10930 = t957 * t10914;
    let t10935 = t238 * t801 * t4261;
    let t10937 = t1392 * t3470;
    let t10939 = t238 * t242 * t10937;
    let t10942 = t238 * t801 * t4265;
    (t10927, t10928, t10930, t10935, t10937, t10939, t10942)
}
