//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1394/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1394<F: Float>(t1882: F, t31825: F, t31640: F, t870: F, t112696: F, t114499: F, t114509: F, t114565: F, t114567: F, t114569: F, t15369: F, t15460: F, t18514: F, t1901: F, t19418: F, t19577: F, t24890: F, t24898: F, t25271: F, t2881: F, t31807: F, t5330: F, t56127: F, t56437: F, t684: F, t98899: F, t99102: F, t99635: F) -> (F,) {
    let t128033 = t1882 * t31825;
    let t128047 = t870 * t31640;
    let t128066 = t114499 + t128033 / 9.0 - 4.0 / 3.0 * t1901 * t15460 * t99102 * t5330 - 4.0 / 3.0 * t1901 * t15460 * t25271 * t19418 + t114509 + 10.0 / 81.0 * t1901 * t56437 * t112696 * t18514 + t99635 + t1901 * t2881 * t128047 * t684 / 9.0 + t1901 * t24890 * t19577 / 9.0 - 4.0 / 3.0 * t1901 * t56127 * t31807 - 4.0 / 3.0 * t1901 * t15369 * t98899 * t5330 - 4.0 / 3.0 * t1901 * t15369 * t24898 * t19418 - t114565 - t114567 - t114569;
    (t128066,)
}
