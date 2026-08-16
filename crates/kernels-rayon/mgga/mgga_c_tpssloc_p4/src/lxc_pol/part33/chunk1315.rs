//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1315/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1315(t25146: f64, t5614: f64, t20949: f64, t6621: f64, t20947: f64, t221: f64, t25154: f64, t20857: f64, t6605: f64, t9972: f64, t105309: f64, t105311: f64, t105313: f64, t105315: f64, t105317: f64, t105319: f64, t105325: f64, t105329: f64, t105333: f64, t105335: f64, t105337: f64, t81850: f64, t81853: f64, t98709: f64, t98711: f64, t98725: f64) -> f64 {
    let t105339 = t25146 * t5614;
    let t105341 = t6621 * t20949;
    let t105345 = t25154 * t221 * t20947;
    let t105348 = t6605 * t9972 * t20857;
    let t105350 = -t105309 / 512.0_f64 + t105311 / 256.0_f64 - t105313 / 128.0_f64 - t105315 / 384.0_f64 - t105317 / 128.0_f64 + 5.0_f64 / 128.0_f64 * t105319 - 7.0_f64 / 16.0_f64 * t98709 - 0.17804385437515912366e0_f64 * t98711 - t81850 - t81853 - 0.60559134141210586281e-3_f64 * t105325 + 0.36335480484726351768e-2_f64 * t105329 + 0.12111826828242117256e-2_f64 * t105333 - t105335 / 1536.0_f64 - t105337 / 512.0_f64 - t105339 / 512.0_f64 + 5.0_f64 / 128.0_f64 * t105341 + 0.42391393898847410397e-2_f64 * t98725 + 3.0_f64 / 16.0_f64 * t105345 - 0.12111826828242117256e-2_f64 * t105348;
    t105350
}
