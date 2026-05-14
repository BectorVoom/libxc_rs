//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1301/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1301<F: Float>(t20922: F, t33633: F, t1520: F, t34888: F, t4170: F, t2282: F, t33702: F, t48691: F, t9831: F, t32226: F, t8286: F, t14294: F, t34849: F, t6241: F, t27047: F, t9509: F) -> (F, F, F, F, F, F, F, F) {
    let t118647 = 4.0 * t20922 * t33633;
    let t118650 = 2.0 * t4170 * t34888 * t1520;
    let t118653 = 4.0 * t4170 * t33702 * t2282;
    let t118655 = 4.0 * t48691 * t9831;
    let t118656 = t32226 * t8286;
    let t118659 = 6.0 * t14294 * t34849 * t1520;
    let t118661 = 2.0 * t6241 * t33702;
    let t118662 = t27047 * t9509;
    (t118647, t118650, t118653, t118655, t118656, t118659, t118661, t118662)
}
