//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 688/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk688<F: Float>(t1073: F, t2266: F, t4458: F, t4462: F, t4883: F, t925: F, t20022: F, t2259: F, t72: F, t20039: F, t3621: F, t21027: F, t21031: F, t21036: F, t21040: F, t21044: F, t21048: F, t2265: F, t631: F) -> (F, F, F, F, F, F, F) {
    let t21052 = t2266 * t4458 * t1073;
    let t21056 = t2266 * t4462 * t1073;
    let t21058 = t925 * t4883;
    let t21059 = t2266 * t21058;
    let t21062 = t72 * t2259 * t20022;
    let t21064 = t3621 * t20039;
    let t21066 = t631 * t21027 / 2.0 + t631 * t21031 / 6.0 + 6.0 * t631 * t21036 - 9.0 / 2.0 * t631 * t21040 + 2.0 / 27.0 * t631 * t21044 + 3.0 * t2265 * t21048 + 2.0 * t2265 * t21052 - t2265 * t21056 - t2265 * t21059 + t631 * t21062 - t2265 * t21064;
    (t21052, t21056, t21058, t21059, t21062, t21064, t21066)
}
