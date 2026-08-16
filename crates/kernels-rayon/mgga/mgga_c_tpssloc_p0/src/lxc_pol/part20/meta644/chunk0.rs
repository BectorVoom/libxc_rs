//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2358/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2358(t13555: f64, t13784: f64, t2986: f64, t13528: f64, t1592: f64, t42891: f64, t973: f64, t13812: f64, t13822: f64, t13881: f64, t13886: f64, t10263: f64, t4506: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48390 = t2986 * t13784 * t13555;
    let t48394 = t2986 * t13784 * t13528;
    let t48397 = t973 * t42891 * t1592;
    let t48402 = t973 * t13822 * t13812;
    let t48407 = t973 * t13822 * t13881;
    let t48417 = t973 * t13822 * t13886;
    let t48421 = t10263 * t4506;
    (t48390, t48394, t48397, t48402, t48407, t48417, t48421)
}
