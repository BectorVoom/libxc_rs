//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1854/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1854(t381: f64, t883: f64, t6743: f64, t14227: f64, t6800: f64, t23384: f64, t6790: f64, t1949: f64, t3010: f64, t6805: f64, t986: f64, t3016: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23634 = t381 * t883;
    let t23635 = t6743 * t23634;
    let t23636 = t14227 * t6800;
    let t23637 = t23635 * t23636;
    let t23642 = t23384 * t6790;
    let t23644 = t3010 * t1949;
    let t23647 = t986 * t6805;
    let t23650 = t3016 * t1949;
    (t23634, t23635, t23637, t23642, t23644, t23647, t23650)
}
