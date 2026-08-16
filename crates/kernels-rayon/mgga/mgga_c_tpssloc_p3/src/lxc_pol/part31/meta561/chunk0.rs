//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1790/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1790(t82147: f64, t1887: f64, t81956: f64, t25041: f64, t215: f64, t6581: f64, t252: f64, t81613: f64, t23056: f64, t25242: f64, t6579: f64, t25245: f64, t82031: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t87042 = 0.52089578783527170489e-1_f64 * t82147;
    let t87049 = t81956 * t1887;
    let t87050 = t87049 * t25041;
    let t87052 = t6581 * t215;
    let t87053 = t81613 * t252;
    let t87057 = t23056 * t215;
    let t87066 = t6579 * t25242;
    let t87068 = t82031 * t25245;
    (t87042, t87049, t87050, t87052, t87053, t87057, t87066, t87068)
}
