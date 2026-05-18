//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1191/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1191<F: Float>(t871: F, t89818: F, t89861: F, t90322: F, t90478: F, t10214: F, t1526: F, t15567: F, t18961: F, t18968: F, t21181: F, t21196: F, t21204: F, t21933: F, t21949: F, t22161: F, t231: F, t2320: F, t342: F, t343: F, t3806: F, t44674: F, t72977: F, t82494: F, t82497: F, t82552: F) -> (F, F) {
    let t90481 = t871 * (t89818 + t89861 + t90322 + t90478);
    let t90516 = -t82552 / F::new(4.0) + F::new(2.0) * t21933 - t1526 * t2320 * t10214 * t21181 / F::new(2.0) + t15567 * t18968 * t21204 / F::new(2.0) + t1526 * t2320 * t21949 / F::new(2.0) + F::new(2.0) / F::new(3.0) * t1526 * t3806 * t44674 * t21181 - t15567 * t18961 * t21196 / F::new(3.0) - t82494 / F::new(12.0) + t82497 / F::new(6.0) - t342 * t343 * t231 * t22161 / F::new(4.0) + t72977 / F::new(6.0);
    (t90481, t90516)
}
