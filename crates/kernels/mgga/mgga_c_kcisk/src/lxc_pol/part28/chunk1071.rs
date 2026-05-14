//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1071/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1071<F: Float>(t24101: F, t24103: F, t24105: F, t24107: F, t24110: F, t24113: F, t24115: F, t24118: F, t24120: F, t24122: F, t24153: F, t24156: F, t24158: F, t24161: F, t24164: F, t24166: F, t24168: F, t24170: F, t24172: F, t24175: F, t24178: F, t24211: F, t24214: F, t24218: F, t24222: F, t24224: F, t24227: F, t24230: F, t24233: F, t24236: F, t24239: F, t24242: F, t24278: F, t24280: F, t24282: F, t24284: F, t24286: F, t24447: F, t24451: F, t24453: F, t24455: F, t24458: F, t24460: F, t24492: F) -> (F,) {
    let t24495 = t24492 + 11.0 / 18.0 * t24460 + t24455 / 18.0 - t24458 / 96.0 - t24451 / 256.0 - 19.0 / 144.0 * t24453 + t24447 / 16.0 - t24286 / 192.0 + t24282 / 256.0 - t24284 / 18.0 + t24278 - t24280 / 192.0 + t24242 / 4.0 - t24239 / 36.0 - t24233 / 24.0 - t24236 / 64.0 - t24230 / 576.0 - t24224 / 96.0 - 3.0 / 8.0 * t24227 + t24222 / 24.0 - t24214 / 12.0 - t24218 / 16.0 + t24211 - t24178 / 72.0 + 2.0 / 27.0 * t24175 - t24170 / 576.0 - t24172 / 18.0 + t24164 / 12.0 + t24166 / 128.0 + t24168 / 96.0 + t24161 / 576.0 + t24156 / 192.0 - t24158 / 12.0 + t24153 + t24120 / 24.0 - t24122 / 12.0 + t24115 / 96.0 + 3.0 / 128.0 * t24118 + t24110 / 12.0 + t24113 / 864.0 - 2.0 / 9.0 * t24105 + t24107 / 3.0 + t24101 / 24.0 - t24103 / 3.0;
    (t24495,)
}
