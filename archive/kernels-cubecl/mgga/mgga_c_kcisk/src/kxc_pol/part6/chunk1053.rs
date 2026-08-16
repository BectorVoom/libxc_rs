//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1053/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1053<F: Float>(t31256: F, t487: F, t486: F, t31212: F, t31215: F, t31218: F, t31220: F, t31223: F, t31226: F, t31229: F, t31232: F, t31235: F, t31238: F, t31241: F, t31243: F, t31245: F, t31248: F, t31250: F, t31252: F, t31254: F) -> (F, F) {
    let t31257 = t487 * t31256;
    let t31258 = t486 * t31257;
    let t31260 = -t31212 / F::cast_from(256.0_f64) - t31215 / F::cast_from(24.0_f64) - t31218 / F::cast_from(32.0_f64) - t31220 / F::cast_from(4.0_f64) + t31223 / F::cast_from(64.0_f64) - t31226 / F::cast_from(192.0_f64) - t31229 / F::cast_from(8.0_f64) - F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t31232 + t31235 / F::cast_from(4.0_f64) + t31238 / F::cast_from(64.0_f64) - t31241 / F::cast_from(8.0_f64) + t31243 / F::cast_from(32.0_f64) - t31245 / F::cast_from(64.0_f64) + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t31248 - F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t31250 + t31252 / F::cast_from(8.0_f64) + F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t31254 + t31258 / F::cast_from(54.0_f64);
    (t31258, t31260)
}
