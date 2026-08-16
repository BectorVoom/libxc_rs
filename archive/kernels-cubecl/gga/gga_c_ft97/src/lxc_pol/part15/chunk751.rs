//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 751/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk751<F: Float>(t4872: F, t8680: F, t925: F, t1073: F, t2266: F, t4458: F, t4462: F, t4883: F, t20022: F, t2259: F, t72: F, t20039: F, t3621: F) -> (F, F, F, F, F, F, F) {
    let t21048 = t8680 * t925 * t4872;
    let t21052 = t2266 * t4458 * t1073;
    let t21056 = t2266 * t4462 * t1073;
    let t21058 = t925 * t4883;
    let t21059 = t2266 * t21058;
    let t21062 = t72 * t2259 * t20022;
    let t21064 = t3621 * t20039;
    (t21048, t21052, t21056, t21058, t21059, t21062, t21064)
}
