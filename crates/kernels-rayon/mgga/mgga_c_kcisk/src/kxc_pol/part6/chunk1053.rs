//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1053/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1053(t31256: f64, t487: f64, t486: f64, t31212: f64, t31215: f64, t31218: f64, t31220: f64, t31223: f64, t31226: f64, t31229: f64, t31232: f64, t31235: f64, t31238: f64, t31241: f64, t31243: f64, t31245: f64, t31248: f64, t31250: f64, t31252: f64, t31254: f64) -> (f64, f64) {
    let t31257 = t487 * t31256;
    let t31258 = t486 * t31257;
    let t31260 = -t31212 / 256.0_f64 - t31215 / 24.0_f64 - t31218 / 32.0_f64 - t31220 / 4.0_f64 + t31223 / 64.0_f64 - t31226 / 192.0_f64 - t31229 / 8.0_f64 - 3.0_f64 / 128.0_f64 * t31232 + t31235 / 4.0_f64 + t31238 / 64.0_f64 - t31241 / 8.0_f64 + t31243 / 32.0_f64 - t31245 / 64.0_f64 + 3.0_f64 / 8.0_f64 * t31248 - 3.0_f64 / 16.0_f64 * t31250 + t31252 / 8.0_f64 + 3.0_f64 / 256.0_f64 * t31254 + t31258 / 54.0_f64;
    (t31258, t31260)
}
