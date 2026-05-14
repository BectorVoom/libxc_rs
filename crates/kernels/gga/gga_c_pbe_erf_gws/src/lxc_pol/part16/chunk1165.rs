//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1165/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1165<F: Float>(t51341: F, t51358: F, t54241: F, t54246: F, t54248: F, t54251: F, t54255: F, t54261: F, t55547: F, t55548: F, t55556: F, t55557: F, t54267: F, t54271: F, t54283: F, t54285: F) -> (F, F, F, F, F) {
    let t55559 = t55547 - t55548 - 7.0 / 36.0 * t51341 + t54241 / 24.0 + t54246 / 12.0 + t54248 / 96.0 - 7.0 / 144.0 * t51358 - t54251 / 8.0 - t54255 / 24.0 + t55556 - t55557 - t54261 / 384.0;
    let t55562 = 7.0 / 36.0 * t54267;
    let t55564 = 7.0 / 72.0 * t54271;
    let t55569 = 7.0 / 288.0 * t54283;
    let t55570 = 7.0 / 72.0 * t54285;
    (t55559, t55562, t55564, t55569, t55570)
}
