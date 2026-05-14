//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1129/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1129<F: Float>(t11687: F, t35790: F, t6951: F, t11682: F, t6943: F, t11683: F, t23579: F, t11632: F, t2245: F, t6201: F, t11633: F, t2208: F, t24181: F, t1062: F, t3728: F, t6935: F) -> (F, F, F, F, F, F) {
    let t35792 = t11687 * t35790 * t6951;
    let t35795 = t11682 * t35790 * t6943;
    let t35798 = t11682 * t11683 * t23579;
    let t35801 = t11632 * t2245 * t6201;
    let t35806 = t24181 * t2208 * t11633;
    let t35809 = t1062 * t3728 * t6935;
    (t35792, t35795, t35798, t35801, t35806, t35809)
}
