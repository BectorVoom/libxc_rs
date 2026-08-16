//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1332/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1332<F: Float>(t11698: F, t24095: F, t1062: F, t3728: F, t6773: F, t11669: F, t2456: F, t11629: F, t11637: F, t1061: F, t23523: F, t6927: F) -> (F, F, F, F, F) {
    let t35956 = t24095 * t11698;
    let t35959 = t1062 * t3728 * t6773;
    let t35962 = t1062 * t11669 * t2456;
    let t35966 = t11637 * t11629;
    let t35970 = t1061 * t23523 * t3728 * t6927;
    (t35956, t35959, t35962, t35966, t35970)
}
