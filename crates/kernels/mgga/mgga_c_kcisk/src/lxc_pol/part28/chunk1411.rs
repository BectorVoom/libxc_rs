//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1411/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1411<F: Float>(t122170: F, t122172: F, t122174: F, t122176: F, t122178: F, t122180: F, t122182: F, t122185: F, t122188: F, t122190: F, t122215: F, t122217: F, t122219: F, t122221: F, t122223: F, t122225: F, t122227: F, t122229: F, t122231: F, t122233: F, t122236: F, t122262: F, t122265: F, t122267: F, t122269: F, t122271: F, t122273: F, t122275: F, t122277: F, t122280: F, t122283: F, t122285: F, t122311: F, t122313: F, t122315: F, t122318: F, t122321: F, t122323: F, t122325: F, t122327: F, t122329: F, t122331: F, t122333: F, t122361: F, t1908: F) -> (F,) {
    let t122365 = t1908 * (-t122223 / 16.0 + t122225 / 48.0 + t122323 / 48.0 + t122325 / 54.0 + t122262 - 19.0 / 72.0 * t122265 + t122267 / 12.0 + t122311 + t122313 / 8.0 - t122170 / 96.0 + t122172 / 128.0 + t122188 / 24.0 + t122285 / 3.0 + t122280 / 6.0 - t122283 / 16.0 + t122227 / 12.0 - t122229 / 64.0 - t122273 / 64.0 + t122275 / 64.0 - t122182 / 128.0 + t122185 / 3.0 + t122215 - t122217 / 48.0 + t122269 / 4.0 - t122271 / 8.0 - t122277 / 96.0 + 2.0 / 27.0 * t122231 + 3.0 / 64.0 * t122233 - t122236 / 32.0 - t122190 / 288.0 + t122361 - t122174 / 3.0 - t122176 / 24.0 - t122331 / 12.0 + t122333 / 48.0 + t122327 / 24.0 + 2.0 / 9.0 * t122329 + t122315 / 96.0 + t122318 / 27.0 + t122178 / 16.0 + t122180 / 8.0 - t122321 / 16.0 - t122219 / 24.0 - t122221 / 12.0);
    (t122365,)
}
