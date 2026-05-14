//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 737/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk737<F: Float>(t109: F, t3972: F, t1620: F, t410: F, t1468: F, t1297: F, t1314: F, t1301: F, t1306: F, t1310: F, t193: F, t202: F, t210: F, t3951: F, t3957: F, t3961: F, t3963: F, t3965: F, t3969: F) -> (F, F, F, F) {
    let t3973 = t109 * t3972;
    let t3974 = t1620 * t410;
    let t3977 = t1468 * t3972;
    let t3978 = t3977 * t1620;
    let t3981 = t1314 * t1297;
    let t3984 = 0.37552696856994557333e-1 * t193 * t3951 * t202 - 0.35400808369803607838e-3 * t1301 * t3957 * t1306 + 0.80569443951744882604e-6 * t3961 * t3963 * t3965 - 40.0 / 9.0 * t1310 * t3969 + 50.0 / 9.0 * t3973 * t3974 + 50.0 / 9.0 * t210 * t3978 - 40.0 / 9.0 * t210 * t3981;
    (t3974, t3978, t3981, t3984)
}
