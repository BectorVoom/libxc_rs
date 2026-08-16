//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 665/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk665<F: Float>(t457: F, t662: F, t3141: F, t481: F, t505: F, t1906: F, t674: F, t682: F, t1927: F, t583: F, t623: F, t1393: F, t515: F) -> (F, F, F, F, F, F, F) {
    let t5726 = t662 * t457;
    let t5727 = t3141 * t5726;
    let t5730 = t481 * t505;
    let t5741 = t1906 * t674;
    let t5742 = t682 * t457;
    let t5743 = t5741 * t5742;
    let t5799 = t1927 * t583;
    let t5803 = t1927 * t623;
    let t5856 = t1393 * t515;
    (t5727, t5730, t5741, t5743, t5799, t5803, t5856)
}
