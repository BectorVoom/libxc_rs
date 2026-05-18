//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1372/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1372<F: Float>(t17353: F, t17354: F, t17301: F, t17304: F, t17308: F, t17311: F, t17333: F, t17337: F, t17339: F, t17340: F, t17342: F, t17344: F, t17347: F, t17351: F, t3674: F, t484: F) -> F {
    let t17355 = t17353 * t17354;
    let t17358 = -t17301 + F::new(0.47637797908966374413e-4) * t17304 + F::new(0.42874018118069736972e-3) * t17308 * t3674 - F::new(0.11433071498151929859e-2) * t17311 * t484 + F::new(0.21437009059034868486e-3) * t17333 * t484 - t17337 + t17339 + F::new(0.2540682555144873302e-3) * t17340 - F::new(0.47637797908966374413e-4) * t17342 - F::new(0.12862205435420921092e-2) * t17344 * t17347 + F::new(0.28582678745379824648e-3) * t17351 * t17355;
    t17358
}
