//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 974/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk974<F: Float>(t19746: F, t3484: F, t19740: F, t13306: F, t5628: F, t1440: F, t220: F, t3797: F, t3796: F, t3748: F, t5613: F, t3739: F, t6003: F, t5882: F, t1284: F, t442: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19747 = t3484 * t19746;
    let t19748 = t19740 * t19747;
    let t19750 = t13306 * t5628;
    let t19752 = t220 * t1440;
    let t19753 = t3797 * t19752;
    let t19754 = t3796 * t19753;
    let t19755 = t19740 * t19754;
    let t19757 = t3748 * t5613;
    let t19759 = t3739 * t6003;
    let t19760 = 0.22109259259259259258e-2 * t19759;
    let t19761 = t3739 * t5882;
    let t19762 = 0.22109259259259259258e-2 * t19761;
    let t19763 = t1284 * t442;
    (t19748, t19750, t19753, t19755, t19757, t19759, t19760, t19761, t19762, t19763)
}
