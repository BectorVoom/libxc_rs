//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1262/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1262<F: Float>(t1882: F, t29359: F, t29329: F, t10703: F, t15157: F, t15175: F, t15354: F, t15358: F, t15369: F, t15460: F, t15533: F, t1901: F, t24873: F, t24886: F, t25271: F, t2843: F, t2844: F, t4167: F, t446: F, t6360: F, t7021: F, t840: F, t99076: F, t99090: F, t99092: F, t99102: F, t99107: F, t99125: F) -> (F,) {
    let t114055 = 4.0 / 9.0 * t1882 * t29359;
    let t114062 = 4.0 / 9.0 * t1882 * t29329;
    let t114078 = -4.0 / 3.0 * t1901 * t15369 * t6360 * t15175 - 4.0 / 3.0 * t1901 * t15460 * t25271 * t15157 - 4.0 / 3.0 * t1901 * t15460 * t99102 * t4167 - t114055 + 2.0 / 27.0 * t99076 - t1901 * t10703 * t24873 * t15533 / 9.0 - t114062 - 2.0 / 3.0 * t446 * t840 * t2843 * t7021 * t2844 - 2.0 / 27.0 * t99090 + 4.0 / 9.0 * t99092 + 2.0 / 9.0 * t1901 * t24886 * t15354 - 2.0 / 9.0 * t99107 - 2.0 / 3.0 * t1901 * t24886 * t15358 - 2.0 / 9.0 * t99125;
    (t114078,)
}
