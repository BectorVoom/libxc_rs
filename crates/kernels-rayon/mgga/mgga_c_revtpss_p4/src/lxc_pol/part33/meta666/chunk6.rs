//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2185/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2185(t22061: f64, t25986: f64, t2661: f64, t22026: f64, t94550: f64, t22052: f64, t7271: f64, t22056: f64, t25972: f64, t94520: f64, t94523: f64, t94526: f64, t94527: f64, t94537: f64, t94540: f64, t94546: f64, t98270: f64) -> f64 {
    let t108601 = t2661 * t25986 * t22061;
    let t108604 = t2661 * t94550 * t22026;
    let t108606 = t7271 * t22052;
    let t108608 = t25972 * t22056;
    let t108613 = -35.0_f64 / 216.0_f64 * t94520 - t94523 + t94526 - 0.60976381323476959248e-3_f64 * t94527 + 0.14291339372689912324e-4_f64 * t108601 - 0.28582678745379824648e-4_f64 * t108604 - 0.17149607247227894789e-2_f64 * t108606 - 0.10164000561857065645e-3_f64 * t108608 + 0.50820002809285328225e-5_f64 * t94537 - 0.36143185997963725434e-4_f64 * t94540 + t98270 - 0.45351183609335988444e-1_f64 * t94546;
    t108613
}
