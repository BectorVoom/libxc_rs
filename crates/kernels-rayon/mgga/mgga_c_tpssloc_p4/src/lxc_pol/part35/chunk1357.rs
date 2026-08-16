//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1357/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1357(t25146: f64, t5619: f64, t5587: f64, t87218: f64, t25068: f64, t5628: f64, t20908: f64, t6621: f64, t1516: f64, t98832: f64, t5624: f64, t232: f64, t6605: f64, t68025: f64, t815: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t105309 = t25146 * t5619;
    let t105311 = t87218 * t5587;
    let t105313 = t25068 * t5628;
    let t105315 = t6621 * t20908;
    let t105317 = t98832 * t1516;
    let t105319 = t25068 * t5624;
    let t105325 = t6605 * t815 * t68025 * t232;
    (t105309, t105311, t105313, t105315, t105317, t105319, t105325)
}
