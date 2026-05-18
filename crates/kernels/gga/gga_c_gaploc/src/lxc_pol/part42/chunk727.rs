//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 727/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk727<F: Float>(t14425: F, t14439: F, t1052: F, t12277: F, t13334: F, t13342: F, t13349: F, t13569: F, t13577: F, t13580: F, t13584: F, t14290: F, t14294: F, t331: F) -> (F, F) {
    let t14440 = t14425 + t14439;
    let t14442 = -F::new(2.0) * t1052 * t12277 + t14440 * t331 + t13334 - t13342 + t13349 - t13569 + t13577 - t13580 - t13584 - t14290 + t14294;
    (t14440, t14442)
}
