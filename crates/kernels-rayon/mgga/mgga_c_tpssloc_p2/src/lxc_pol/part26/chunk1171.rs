//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1171/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1171(t3243: f64, t7363: f64, t24776: f64, t2148: f64, t3471: f64, t3616: f64, t7376: f64, t7375: f64, t225: f64, t7319: f64, t7364: f64, t24757: f64, t493: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24777 = t7363 * t3243;
    let t24778 = t24776 * t24777;
    let t24781 = t3471 * t2148;
    let t24784 = t3616 * t7376;
    let t24785 = t7375 * t24784;
    let t24788 = t7319 * t225;
    let t24789 = t24788 * t7364;
    let t24792 = t493 * t24757;
    (t24777, t24778, t24781, t24784, t24785, t24788, t24789, t24792)
}
