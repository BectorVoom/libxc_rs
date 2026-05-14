//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 930/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk930<F: Float>(t2259: F, t8256: F, t6382: F, t8275: F, t30489: F, t4143: F, t487: F, t486: F, t31212: F, t31215: F, t31218: F, t31220: F, t31223: F, t31226: F, t31229: F, t31232: F, t31235: F, t31238: F, t31241: F, t31243: F, t31245: F, t31248: F, t31250: F) -> (F, F, F, F) {
    let t31252 = t2259 * t8256;
    let t31254 = t6382 * t8275;
    let t31256 = t4143 * t30489;
    let t31257 = t487 * t31256;
    let t31258 = t486 * t31257;
    let t31260 = -t31212 / 256.0 - t31215 / 24.0 - t31218 / 32.0 - t31220 / 4.0 + t31223 / 64.0 - t31226 / 192.0 - t31229 / 8.0 - 3.0 / 128.0 * t31232 + t31235 / 4.0 + t31238 / 64.0 - t31241 / 8.0 + t31243 / 32.0 - t31245 / 64.0 + 3.0 / 8.0 * t31248 - 3.0 / 16.0 * t31250 + t31252 / 8.0 + 3.0 / 256.0 * t31254 + t31258 / 54.0;
    (t31252, t31254, t31258, t31260)
}
