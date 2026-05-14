//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1101/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1101<F: Float>(t1146: F, t2881: F, t2995: F, t3570: F, t3781: F, t44882: F, t44885: F, t44888: F, t44893: F, t44897: F, t44899: F, t44902: F, t44904: F, t44907: F, t44909: F, t44912: F, t44915: F, t44918: F, t44921: F, t9832: F) -> (F,) {
    let t44922 = t1146 * t9832 + 2.0 * t2881 * t3781 + t2995 * t3570 - t44882 - t44885 - t44888 - t44893 - t44897 - t44899 + t44902 + t44904 + t44907 + t44909 - t44912 + t44915 - t44918 + t44921;
    (t44922,)
}
