//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1271/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1271<F: Float>(t2567: F, t6837: F, t31170: F, t8392: F, t31167: F, t110629: F, t110669: F, t110931: F, t110933: F, t110946: F, t110950: F, t110961: F, t111089: F, t13885: F, t14127: F, t18433: F, t18438: F, t18442: F, t1901: F, t31106: F, t3859: F, t3864: F, t3881: F, t53797: F, t54032: F, t9787: F, t97870: F, t97872: F) -> (F,) {
    let t124402 = t2567 * t6837;
    let t124425 = t8392 * t31170;
    let t124427 = t8392 * t31167;
    let t124432 = -4.0 / 3.0 * t1901 * t14127 * t124402 * t3864 + 2.0 / 9.0 * t1901 * t110950 * t3881 - 4.0 / 3.0 * t1901 * t13885 * t110629 * t3859 - 4.0 / 27.0 * t97870 - 4.0 / 27.0 * t97872 - t110931 - t110933 + 4.0 / 9.0 * t53797 * t110669 * t18433 + 4.0 / 9.0 * t53797 * t111089 * t18438 - 4.0 / 27.0 * t54032 * t111089 * t18442 - t110946 - 2.0 / 27.0 * t124425 - 2.0 / 27.0 * t124427 + t1901 * t9787 * t31106 / 9.0 + t110961;
    (t124432,)
}
