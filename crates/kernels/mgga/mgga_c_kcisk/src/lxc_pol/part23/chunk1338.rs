//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1338/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1338<F: Float>(t113478: F, t9492: F, t21090: F, t9491: F, t109293: F, t3784: F, t6370: F, t32255: F, t33658: F, t113446: F, t113448: F, t113450: F, t113452: F, t113454: F, t113456: F, t113458: F, t113460: F, t113462: F, t113464: F, t113466: F, t113468: F, t113470: F, t113472: F, t113474: F, t113476: F) -> (F, F, F, F, F) {
    let t113479 = t113478 * t9492;
    let t113481 = t9491 * t21090;
    let t113484 = t3784 * t109293 * t6370;
    let t113486 = t32255 * t33658;
    let t113488 = t113446 / 64.0 - 19.0 / 54.0 * t113448 - 2.0 / 9.0 * t113450 - 3.0 / 8.0 * t113452 + t113454 / 48.0 + 19.0 / 72.0 * t113456 + t113458 / 4.0 - t113460 / 72.0 + t113462 / 432.0 - t113464 / 12.0 - t113466 / 9.0 - t113468 / 24.0 - t113470 / 16.0 - t113472 / 8.0 - t113474 / 16.0 - t113476 / 96.0 + t113479 / 3.0 + t113481 / 72.0 + t113484 / 6.0 + t113486 / 3.0;
    (t113479, t113481, t113484, t113486, t113488)
}
