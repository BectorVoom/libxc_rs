//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 720/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk720<F: Float>(t21181: F, t9953: F, t9952: F, t2487: F, t737: F, t21204: F, t3917: F, t21453: F, t2493: F, t21457: F, t21442: F, t9916: F, t9920: F, t2486: F, t21570: F, t21573: F, t21577: F, t21581: F, t462: F, t92: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t21584 = t9953 * t21181;
    let t21585 = t9952 * t21584;
    let t21588 = t2487 * t21181;
    let t21589 = t737 * t21588;
    let t21592 = t3917 * t21204;
    let t21595 = t2493 * t21453;
    let t21597 = t2493 * t21457;
    let t21599 = t9916 * t21442;
    let t21602 = t9920 * t21181;
    let t21603 = t2486 * t21602;
    let t21606 = -t92 * t21570 - t462 * t21573 / 3.0 - 6.0 * t92 * t21577 + 6.0 * t462 * t21581 - 10.0 / 27.0 * t462 * t21585 - 2.0 * t462 * t21589 + 2.0 * t462 * t21592 + t462 * t21595 + t462 * t21597 + 2.0 / 3.0 * t462 * t21599 + 4.0 / 3.0 * t462 * t21603;
    (t21584, t21585, t21588, t21589, t21592, t21595, t21597, t21599, t21602, t21603, t21606)
}
