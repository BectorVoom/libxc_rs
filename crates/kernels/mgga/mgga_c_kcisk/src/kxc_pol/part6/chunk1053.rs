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
    let t31260 = -t31212 / F::new(256.0) - t31215 / F::new(24.0) - t31218 / F::new(32.0) - t31220 / F::new(4.0) + t31223 / F::new(64.0) - t31226 / F::new(192.0) - t31229 / F::new(8.0) - F::new(3.0) / F::new(128.0) * t31232 + t31235 / F::new(4.0) + t31238 / F::new(64.0) - t31241 / F::new(8.0) + t31243 / F::new(32.0) - t31245 / F::new(64.0) + F::new(3.0) / F::new(8.0) * t31248 - F::new(3.0) / F::new(16.0) * t31250 + t31252 / F::new(8.0) + F::new(3.0) / F::new(256.0) * t31254 + t31258 / F::new(54.0);
    (t31258, t31260)
}
