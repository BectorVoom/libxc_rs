//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1409/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1409<F: Float>(t1882: F, t31762: F, t112746: F, t112987: F, t114104: F, t114531: F, t114841: F, t114843: F, t114852: F, t125696: F, t15369: F, t15460: F, t19002: F, t19006: F, t1901: F, t19362: F, t19367: F, t19593: F, t19617: F, t19815: F, t25271: F, t25368: F, t296: F, t4139: F, t4176: F, t446: F, t4965: F, t53797: F, t6360: F, t6361: F, t7036: F, t72391: F, t98966: F) -> (F,) {
    let t128462 = t1882 * t31762;
    let t128495 = t114841 - t114843 - 4.0 / 9.0 * t1901 * t112987 * t19002 + 4.0 / 27.0 * t1901 * t112746 * t19006 + t128462 / 9.0 + 4.0 / 9.0 * t53797 * t98966 * t19815 + 4.0 / 9.0 * t53797 * t98966 * t19593 + 4.0 / 3.0 * t53797 * t114104 * t19617 + t114852 - t446 * t296 * t125696 / 3.0 - 2.0 / 3.0 * t1901 * t15369 * t6360 * t19362 - 2.0 / 3.0 * t1901 * t15460 * t25271 * t19367 + t1901 * t72391 * t6361 / 9.0 + 2.0 / 27.0 * t1901 * t4139 * t25368 * t4965 + 4.0 * t1901 * t114531 * t7036 * t4176;
    (t128495,)
}
