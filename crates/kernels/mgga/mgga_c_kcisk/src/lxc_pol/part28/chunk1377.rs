//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1377/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1377<F: Float>(t117182: F, t24027: F, t5182: F, t112586: F, t24031: F, t116536: F, t23986: F, t23990: F, t6674: F, t116223: F, t1799: F, t6986: F, t112176: F, t8480: F, t22369: F, t33017: F) -> (F, F, F, F, F, F, F) {
    let t121730 = t5182 * t117182 * t24027;
    let t121733 = t5182 * t112586 * t24031;
    let t121736 = t5182 * t116536 * t23986;
    let t121739 = t6674 * t116536 * t23990;
    let t121748 = t1799 * t116223 * t6986;
    let t121751 = t1799 * t112176 * t8480;
    let t121754 = t1799 * t33017 * t22369;
    (t121730, t121733, t121736, t121739, t121748, t121751, t121754)
}
