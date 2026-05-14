//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1413/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1413<F: Float>(t35337: F, t5213: F, t34288: F, t7444: F, t10028: F, t117560: F, t120987: F, t120990: F, t120993: F, t120995: F, t120997: F, t120999: F, t121001: F, t121004: F, t121006: F, t121008: F, t12345: F, t12352: F, t18925: F, t2049: F, t25271: F, t2666: F, t2815: F, t34618: F, t35344: F, t35378: F, t35526: F, t48510: F, t5532: F, t7690: F) -> (F, F, F) {
    let t122380 = t5213 * t35337;
    let t122382 = 2.0 * t34288 * t7444;
    let t122403 = -12.0 * t10028 * t12352 * t7690 - 6.0 * t12352 * t2049 * t35344 - 12.0 * t12352 * t2049 * t35378 + 2.0 * t25271 * t2815 * t5532 - 2.0 * t117560 * t2666 + 4.0 * t12345 * t35378 + 4.0 * t18925 * t34618 - 6.0 * t35526 * t48510 + t120987 + t120990 - t120993 - t120995 - t120997 - t120999 - t121001 - t121004 - t121006 - t121008;
    (t122380, t122382, t122403)
}
