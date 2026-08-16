//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 823/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk823(t25277: f64, t25077: f64, t25080: f64, t23114: f64, t23120: f64, t24218: f64, t24220: f64, t24221: f64, t25085: f64, t25087: f64, t25089: f64, t25091: f64, t25095: f64, t25099: f64) -> (f64, f64, f64, f64) {
    let t26613 = 0.38381794893125283518e-1_f64 * t25277;
    let t26619 = 7.0_f64 / 288.0_f64 * t25077;
    let t26621 = 7.0_f64 / 1152.0_f64 * t25080;
    let t26630 = t24218 - t24220 + t25085 / 384.0_f64 + t25087 / 192.0_f64 - t25089 / 768.0_f64 + t25091 / 192.0_f64 + 0.80745512188280781706e-3_f64 * t25095 + t24221 + 0.24223653656484234512e-2_f64 * t25099 + 0.67287926823567318088e-4_f64 * t23114 - t23120;
    (t26613, t26619, t26621, t26630)
}
