//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1046/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1046<F: Float>(t23682: F, t23685: F, t23651: F, t23653: F, t23655: F, t23660: F, t23664: F, t23667: F, t23670: F, t23673: F, t23676: F, t23679: F, t2296: F, t2301: F, t2302: F, t2315: F, t23578: F, t23618: F, t23649: F, t23687: F, t23691: F, t23694: F, t23699: F, t23708: F, t23709: F, t23715: F, t23732: F, t23745: F, t23758: F, t350: F, t8335: F, t8338: F, t8345: F, t8346: F, t8349: F, t8376: F, t974: F, t979: F) -> (F,) {
    let t23769 = 0.75383950617283950617e4 * t23682;
    let t23770 = 0.12819753086419753086e4 * t23685;
    let t23771 = 0.47199999999999999999e3 * t23651 - 0.58153333333333333333e4 * t23653 + 0.19384444444444444445e4 * t23655 + 0.58153333333333333332e4 * t23660 - 2832.0 * t23664 + 0.62933333333333333332e3 * t23667 + 17446.0 * t23670 - 0.19384444444444444444e4 * t23673 - 0.4846111111111111111e4 * t23676 - 26169.0 * t23679 + t23769 + t23770;
    let t23775 = (t23578 + t23618 + t23649 + t23687) * t350 - 4.0 * t23691 * t979 + 12.0 * t23694 * t2302 - 6.0 * t8335 * t2315 - 24.0 * t23699 * t8346 + 24.0 * t8338 * t8349 - 4.0 * t2296 * t8376 + 24.0 * t23708 * t23709 - 36.0 * t8345 * t2302 * t2315 + 6.0 * t2301 * t23715 + 8.0 * t2301 * t979 * t8376 - t974 * (t23732 + t23745 + t23758 + t23771);
    (t23775,)
}
