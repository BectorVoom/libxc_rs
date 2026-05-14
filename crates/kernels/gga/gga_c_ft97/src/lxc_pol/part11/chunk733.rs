//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 733/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk733<F: Float>(t10861: F, t2923: F, t230: F, t2938: F, t2939: F, t684: F, t10835: F, t10838: F, t10839: F, t10841: F, t10843: F, t10847: F, t10852: F, t10855: F, t10859: F, t2265: F, t631: F) -> (F, F, F, F) {
    let t10862 = t2923 * t10861;
    let t10864 = t230 * t2938;
    let t10865 = t684 * t2939;
    let t10866 = t10864 * t10865;
    let t10869 = -t10835 + t10838 + 5.0 / 3.0 * t10839 + 4.0 / 3.0 * t10841 + 2.0 / 3.0 * t10843 - t2265 * t10847 / 3.0 - t631 * t10852 / 3.0 + t2265 * t10855 / 6.0 - t2265 * t10859 - t2265 * t10862 + 3.0 * t2265 * t10866;
    (t10862, t10864, t10866, t10869)
}
