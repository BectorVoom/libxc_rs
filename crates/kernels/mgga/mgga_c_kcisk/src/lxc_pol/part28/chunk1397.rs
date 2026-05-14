//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1397/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1397<F: Float>(t117563: F, t2594: F, t34300: F, t65005: F, t11701: F, t1957: F, t35280: F, t24094: F, t33071: F, t35274: F, t47024: F, t65157: F, t9699: F, t33068: F, t9094: F, t120987: F, t120990: F, t120993: F, t120995: F, t120997: F, t120999: F, t121001: F, t121004: F, t121006: F, t121008: F, t122139: F) -> (F, F, F, F, F, F, F, F) {
    let t122141 = 2.0 * t117563 * t2594;
    let t122143 = 12.0 * t65005 * t34300;
    let t122146 = 6.0 * t11701 * t35280 * t1957;
    let t122148 = 4.0 * t33071 * t24094;
    let t122150 = 6.0 * t47024 * t35274;
    let t122152 = 2.0 * t65157 * t9699;
    let t122153 = t33068 * t9094;
    let t122154 = -t120987 - t120990 + t120993 + t120995 + t120997 + t120999 + t121001 + t121004 + t121006 + t121008 + t122139 - t122141 - t122143 - t122146 + t122148 - t122150 + t122152 - t122153;
    (t122141, t122143, t122146, t122148, t122150, t122152, t122153, t122154)
}
