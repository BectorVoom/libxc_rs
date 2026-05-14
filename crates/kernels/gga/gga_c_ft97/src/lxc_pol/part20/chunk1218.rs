//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1218/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1218<F: Float>(t113066: F, t24976: F, t6317: F, t10570: F, t192: F, t24980: F, t25178: F, t7036: F, t113035: F, t113039: F, t113043: F, t113046: F, t113049: F, t113053: F, t113058: F, t113061: F, t113064: F) -> (F, F, F) {
    let t113068 = t6317 * t24976 * t113066;
    let t113070 = t192 * t10570;
    let t113073 = t24980 * t113070 * t7036 * t25178;
    let t113075 = -t113035 / 6.0 - t113039 / 6.0 - t113043 / 3.0 - t113046 / 3.0 - 2.0 / 3.0 * t113049 + t113053 / 9.0 + 5.0 / 27.0 * t113058 + t113061 + 2.0 / 3.0 * t113064 - 2.0 / 3.0 * t113068 + 3.0 * t113073;
    (t113068, t113073, t113075)
}
