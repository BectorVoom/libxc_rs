//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 819/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk819<F: Float>(t1882: F, t5079: F, t5075: F, t17720: F, t17724: F, t17729: F, t17734: F, t17738: F, t17742: F, t17746: F, t17751: F, t17755: F, t17759: F, t17763: F, t13722: F, t13732: F, t14317: F, t14318: F, t17768: F, t17773: F, t17778: F, t17782: F, t17787: F, t17792: F, t17796: F) -> (F, F, F, F) {
    let t18542 = t1882 * t5079;
    let t18544 = t1882 * t5075;
    let t18557 = -2.0 / 27.0 * t17720 + t17724 / 9.0 + 2.0 / 9.0 * t17729 - 2.0 / 27.0 * t17734 - 4.0 / 9.0 * t17738 - 2.0 / 9.0 * t17742 - 2.0 / 3.0 * t17746 - 10.0 / 81.0 * t17751 + 8.0 / 27.0 * t17755 + 2.0 / 9.0 * t17759 + 2.0 / 27.0 * t17763;
    let t18567 = 4.0 / 9.0 * t17768 + t17773 / 9.0 - 2.0 / 9.0 * t17778 - 8.0 / 9.0 * t17782 - t14317 - 4.0 / 9.0 * t17787 - 4.0 / 9.0 * t17792 + 4.0 / 27.0 * t17796 - t14318 - 8.0 / 81.0 * t13722 - 4.0 / 27.0 * t13732;
    (t18542, t18544, t18557, t18567)
}
