//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1278/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1278<F: Float>(t124144: F, t124148: F, t108345: F, t108354: F, t108357: F, t108394: F, t108430: F, t124127: F, t124133: F, t124137: F, t124141: F, t97360: F, t124172: F, t108432: F, t108434: F, t109357: F, t109359: F, t109361: F, t124154: F, t124157: F, t124160: F, t124164: F, t124169: F, t124177: F) -> (F, F) {
    let t124600 = 2.0 / 3.0 * t124144;
    let t124601 = 4.0 / 3.0 * t124148;
    let t124602 = 2.0 * t124127 + 2.0 / 9.0 * t108345 + t124133 / 3.0 - 12.0 * t124137 + 4.0 * t124141 - t124600 + t97360 - t108354 - t108357 + t108394 - t124601 + t108430;
    let t124607 = t124172 / 12.0;
    let t124610 = t108432 - t108434 + t124154 - 2.0 / 9.0 * t124157 + 5.0 / 27.0 * t124160 - 2.0 / 3.0 * t124164 + t124169 / 2.0 - t124607 + t124177 / 2.0 + t109357 + t109359 + 2.0 / 9.0 * t109361;
    (t124602, t124610)
}
