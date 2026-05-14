//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1100/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1100<F: Float>(t2041: F, t9258: F, t2049: F, t9262: F, t2666: F, t7690: F, t9291: F, t24126: F, t24128: F, t24130: F, t24132: F, t24135: F, t24138: F, t24141: F, t24144: F, t24147: F, t24149: F, t24151: F) -> (F, F, F, F, F) {
    let t25153 = t9258 * t2041;
    let t25163 = t9262 * t2049;
    let t25166 = t2666 * t7690;
    let t25170 = t9291 * t2049;
    let t25195 = 0.47962962962962962963e-1 * t24126 - 0.10791666666666666667e0 * t24128 + 0.5e0 * t24130 + 0.33333333333333333333e0 * t24132 + 0.5e0 * t24135 - 0.4046875e-1 * t24138 + 0.375e0 * t24141 - 0.91666666666666666667e0 * t24144 - 0.125e0 * t24147 - 0.20833333333333333333e-1 * t24149 - 0.1875e0 * t24151;
    (t25153, t25163, t25166, t25170, t25195)
}
