//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1590/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1590(t1860: f64, t23993: f64, t6509: f64, t7031: f64, t22819: f64, t22825: f64, t22858: f64, t22863: f64, t22867: f64, t22645: f64, t225: f64, t7192: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23995 = 88.0_f64 / 27.0_f64 * t1860 * t23993;
    let t23998 = t7031 * t6509;
    let t23999 = t1860 * t23998;
    let t24049 = 0.33643963411783659044e-4_f64 * t22819;
    let t24050 = 0.10541775202358879834e-2_f64 * t22825;
    let t24058 = 119.0_f64 / 3456.0_f64 * t22858;
    let t24060 = 35.0_f64 / 216.0_f64 * t22863;
    let t24061 = 0.22608743412718618878e-1_f64 * t22867;
    let t24071 = 0.16449340668482264365e-1_f64 * t22645;
    let t24082 = t7192 * t225;
    (t23995, t23998, t23999, t24049, t24050, t24058, t24060, t24061, t24071, t24082)
}
