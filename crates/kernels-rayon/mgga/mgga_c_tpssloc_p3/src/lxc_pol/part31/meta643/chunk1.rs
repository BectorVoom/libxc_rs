//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1912/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1912(t23270: f64, t258: f64, t5527: f64, t776: f64, t87642: f64, t6552: f64, t7479: f64, t87782: f64, t2717: f64, t5636: f64, t22986: f64, t5544: f64) -> (f64, f64, f64, f64) {
    let t98153 = t87642 * t23270 * t258 * t5527 * t776;
    let t98158 = t6552 * t87782 * t7479;
    let t98161 = t2717 * t5636;
    let t98164 = t22986 * t23270 * t98161 * t776;
    let t98169 = t258 * t5544;
    (t98153, t98158, t98164, t98169)
}
