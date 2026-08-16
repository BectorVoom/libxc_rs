//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 827/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk827(t23095: f64, t23105: f64, t23107: f64, t23140: f64, t23143: f64, t23100: f64, t23114: f64, t23117: f64, t23119: f64, t23125: f64, t23128: f64, t23130: f64, t23134: f64, t23136: f64, t23147: f64) -> f64 {
    let t24218 = 0.10541775202358879834e-2_f64 * t23095;
    let t24220 = 0.33643963411783659044e-4_f64 * t23105;
    let t24221 = 119.0_f64 / 3456.0_f64 * t23107;
    let t24230 = 0.22608743412718618878e-1_f64 * t23140;
    let t24231 = 35.0_f64 / 216.0_f64 * t23143;
    let t24233 = t24218 + 0.48447307312968469024e-2_f64 * t23100 - t24220 + t24221 + 0.13457585364713463618e-3_f64 * t23114 + t23117 / 768.0_f64 - 7.0_f64 / 576.0_f64 * t23119 + 0.80745512188280781706e-3_f64 * t23125 - t23128 / 96.0_f64 + 5.0_f64 / 192.0_f64 * t23130 + 7.0_f64 / 144.0_f64 * t23134 - t23136 / 192.0_f64 + t24230 + t24231 + t23147 / 96.0_f64;
    t24233
}
