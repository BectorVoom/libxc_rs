//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 902/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk902<F: Float>(t557: F, t92: F, t125: F, t2994: F, t547: F, t1232: F, t1815: F, t19: F, t222: F, t3: F, t6610: F) -> (F, F, F, F, F) {
    let t7873 = t557 * t92;
    let t7874 = t7873 * t125;
    let t7879 = t547 * t2994 / F::cast_from(32.0_f64);
    let t7880 = t1815 * t1232;
    let t7881 = t19 * t7880;
    let t7884 = t3 * t6610 * t222;
    (t7874, t7879, t7880, t7881, t7884)
}
