//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 408/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk408<F: Float>(t2591: F, t741: F, t2561: F, t2565: F, t2569: F, t2573: F, t2577: F, t2581: F, t2588: F) -> (F, F) {
    let t2592 = t741 * t2591;
    let t2594 = t2561 / 16.0 - t2565 / 16.0 - t2569 / 6.0 + t2573 / 24.0 - t2577 / 256.0 + t2581 / 256.0 + t2588 / 48.0 - t2592 / 192.0;
    (t2592, t2594)
}
