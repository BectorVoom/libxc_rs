//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1363/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1363(t20904: f64, t81818: f64, t20953: f64, t6614: f64, t20978: f64, t23146: f64, t20861: f64, t2628: f64, t6605: f64, t105387: f64, t105390: f64, t105393: f64, t105396: f64, t105402: f64, t81921: f64, t81955: f64, t87387: f64, t87403: f64, t87405: f64, t87432: f64, t87445: f64, t98828: f64, t98830: f64, t98836: f64, t98838: f64) -> f64 {
    let t105404 = t81818 * t20904;
    let t105406 = t6614 * t20953;
    let t105412 = t23146 * t20978;
    let t105415 = t6605 * t2628 * t20861;
    let t105417 = -0.25434836339308446237e-1_f64 * t105387 + 0.25434836339308446237e-1_f64 * t105390 - 0.20186378047070195427e-3_f64 * t105393 - 0.94875976821229918508e-2_f64 * t87387 - 5.0_f64 / 64.0_f64 * t105396 + 119.0_f64 / 2304.0_f64 * t87403 - 0.15812662803538319751e-2_f64 * t87405 - 35.0_f64 / 192.0_f64 * t98828 + 7.0_f64 / 96.0_f64 * t98830 + t105402 / 256.0_f64 - t105404 / 256.0_f64 - t105406 / 1536.0_f64 - 0.84782787797694820794e-2_f64 * t98836 - 0.33913115119077928317e-1_f64 * t87432 - 0.50869672678616892476e-1_f64 * t98838 - t81921 + 0.3027956707060529314e-3_f64 * t87445 - t81955 + t105412 / 128.0_f64 + 0.12111826828242117256e-2_f64 * t105415;
    t105417
}
