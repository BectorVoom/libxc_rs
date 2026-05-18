//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1238/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1238<F: Float>(t2311: F, t3396: F, t1363: F, t6666: F, t820: F, t8810: F, t2272: F, t3363: F, t1351: F, t6709: F, t1358: F, t2273: F) -> (F, F, F, F, F, F) {
    let t24848 = t3396 * t2311;
    let t24896 = t1363 * t6666;
    let t24911 = t8810 * t820;
    let t24916 = t3363 * t2272;
    let t24923 = t1351 * t6709;
    let t24926 = t2273 * t1358;
    (t24848, t24896, t24911, t24916, t24923, t24926)
}
