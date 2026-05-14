//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1108/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1108<F: Float>(t3186: F, t32552: F, t1009: F, t9335: F, t1053: F, t15723: F, t31861: F, t31864: F, t31866: F, t31876: F, t31885: F, t31999: F, t32543: F, t32546: F, t32549: F, t3442: F, t3443: F) -> (F, F, F, F) {
    let t32553 = t3186 * t32552;
    let t32554 = 2.0 * t32553;
    let t32555 = t9335 * t1009;
    let t32556 = t32555 * t1053;
    let t32557 = 2.0 * t32556;
    let t32558 = -6.0 * t15723 * t32543 + 2.0 * t32546 * t3442 + 2.0 * t32549 * t3443 + t31861 - t31864 + t31866 - t31876 - t31885 + t31999 - t32554 + t32557;
    (t32553, t32555, t32556, t32558)
}
