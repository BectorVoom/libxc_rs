//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1951/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1951(t1408: f64, t4255: f64, t870: f64, t25365: f64, t57911: f64, t10143: f64, t1484: f64, t25374: f64, t23788: f64, t67128: f64, t16949: f64, t25891: f64) -> (f64, f64, f64, f64, f64) {
    let t99060 = t870 * t1408 * t4255;
    let t100562 = t57911 * t25365;
    let t100572 = t10143 * t1484 * t25374;
    let t100638 = t23788 * t67128;
    let t100641 = t25891 * t16949;
    (t99060, t100562, t100572, t100638, t100641)
}
