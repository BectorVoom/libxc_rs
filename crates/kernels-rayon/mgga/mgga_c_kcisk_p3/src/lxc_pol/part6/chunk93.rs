//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 93/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk93(t311: f64, t312: f64, t313: f64, t303: f64, t306: f64, t309: f64, t305: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t315 = t311 * t312 * t313;
    let t317 = 0.379785e1_f64 * t306 + 0.8969e0_f64 * t303 + 0.204775e0_f64 * t309 + 0.123235e0_f64 * t315;
    let t320 = 1.0_f64 + 0.16081824322151104822e2_f64 / t317;
    let t321 = f64::ln(t320);
    let t323 = 0.62182e-1_f64 * t305 * t321;
    let t325 = 1.0_f64 + 0.278125e-1_f64 * t303;
    (t315, t317, t320, t321, t323, t325)
}
