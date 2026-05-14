//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 454/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk454<F: Float>(t2801: F, t871: F, t296: F, t824: F, t840: F, t882: F, t2739: F, t319: F, t2399: F, t313: F, t89: F, t1882: F, t842: F, t877: F, t2652: F, t2655: F, t2658: F, t2663: F, t2668: F, t2673: F, t2677: F, t2685: F, t2742: F, t2758: F, t2791: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2802 = t871 * t2801;
    let t2803 = t296 * t2802;
    let t2807 = t840 * t882 * t824;
    let t2811 = t840 * t319 * t2739;
    let t2816 = 4.0 / 27.0 * t89 * t2399 * t313;
    let t2817 = t1882 * t842;
    let t2819 = t1882 * t877;
    let t2823 = 4.0 / 27.0 * t2652;
    let t2832 = -t2758 / 12.0 + t2791 / 6.0 + t2823 + 2.0 / 27.0 * t2655 + 2.0 / 9.0 * t2658 - 2.0 / 27.0 * t2663 + 2.0 / 9.0 * t2668 + 2.0 / 9.0 * t2673 - t2677 / 9.0 + 2.0 / 3.0 * t2685 - t2742 / 3.0;
    (t2802, t2803, t2807, t2811, t2816, t2817, t2819, t2823, t2832)
}
