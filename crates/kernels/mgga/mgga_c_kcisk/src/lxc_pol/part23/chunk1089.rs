//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1089/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1089<F: Float>(t1550: F, t19473: F, t19482: F, t19559: F, t19577: F, t19582: F, t19585: F, t19587: F, t19602: F, t19604: F, t19606: F, t19609: F, t19612: F, t19615: F, t19619: F, t19622: F, t19625: F, t19644: F, t21745: F, t21821: F, t21838: F, t21875: F, t240: F) -> (F,) {
    let t21883 = t19482 + t19559 - t19582 + t19585 + t19587 + t240 * (t21745 + t21821 + t21838 + t21875) + 0.19751789702565206229e-1 * t240 * t19577 + t19602 - t19604 + t19606 - t19609 - t19612 - t19615 + t19619 + t19622 + t19625 - 0.34631511798751726598e2 * t1550 * t19473 - t19644;
    (t21883,)
}
