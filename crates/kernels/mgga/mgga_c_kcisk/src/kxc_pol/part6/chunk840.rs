//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 840/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk840<F: Float>(t29613: F, t734: F, t28963: F, t747: F, t746: F, t11927: F, t2560: F, t9058: F, t2576: F, t9066: F, t7337: F, t9043: F, t29586: F, t29589: F, t29591: F, t29595: F, t29598: F, t29601: F, t29603: F, t29607: F, t29609: F, t29611: F) -> (F, F, F, F, F, F) {
    let t29614 = t734 * t29613;
    let t29616 = t747 * t28963;
    let t29617 = t746 * t29616;
    let t29618 = t11927 * t29617;
    let t29620 = t2560 * t9058;
    let t29622 = t2576 * t9066;
    let t29624 = t7337 * t9043;
    let t29625 = -t29586 / 256.0 - t29589 + t29591 / 8.0 + t29595 / 54.0 + t29598 / 192.0 - t29601 / 16.0 + t29603 / 8.0 - 3.0 / 8.0 * t29607 - t29609 / 8.0 - t29611 / 64.0 + t29614 / 9.0 + 3.0 / 128.0 * t29618 - 2.0 / 3.0 * t29620 - t29622 / 192.0 + t29624;
    (t29614, t29618, t29620, t29622, t29624, t29625)
}
