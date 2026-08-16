//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1373/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1373(t10914: f64, t2484: f64, t952: f64, t10892: f64, t2490: f64, t10921: f64, t3496: f64, t9135: f64, t2496: f64, t10927: f64, t3490: f64, t9151: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29851 = t2484 * t10914 * t952;
    let t29853 = t10892 * t2490;
    let t29855 = t10921 * t2490;
    let t29857 = t3496 * t9135;
    let t29860 = t2496 * t10914 * t952;
    let t29862 = t10927 * t2490;
    let t29864 = t952 * t3490;
    let t29865 = t9151 * t29864;
    (t29851, t29853, t29855, t29857, t29860, t29862, t29864, t29865)
}
