//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1164/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1164<F: Float>(t51285: F, t51293: F, t51302: F, t51315: F, t51330: F, t51332: F, t54215: F, t54217: F, t54219: F, t54224: F, t54226: F, t54231: F, t54236: F, t54238: F, t54257: F, t54259: F) -> (F, F, F, F, F) {
    let t55546 = -t54215 / 48.0 + t54217 / 192.0 + t54219 / 384.0 - 7.0 / 576.0 * t51285 + 7.0 / 36.0 * t51293 - 7.0 / 192.0 * t51302 - t54224 / 96.0 + 5.0 / 192.0 * t54226 - 7.0 / 288.0 * t51315 - t54231 / 24.0 + 7.0 / 144.0 * t51330 - 7.0 / 576.0 * t51332;
    let t55547 = 7.0 / 72.0 * t54236;
    let t55548 = 7.0 / 144.0 * t54238;
    let t55556 = 7.0 / 72.0 * t54257;
    let t55557 = 7.0 / 36.0 * t54259;
    (t55546, t55547, t55548, t55556, t55557)
}
