//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1428/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1428<F: Float>(t12622: F, t1611: F, t12590: F, t4908: F, t33212: F, t33217: F, t33228: F, t36508: F, t36510: F, t36512: F, t36513: F, t36515: F, t36516: F, t36517: F, t36518: F) -> (F, F, F) {
    let t38708 = F::new(2.0) * t1611 * t12622;
    let t38710 = F::new(4.0) * t4908 * t12590;
    let t38714 = t36508 + F::new(0.36231816839129402172e-6) * t33212 - t36510 + F::new(0.18115908419564701086e-6) * t33217 + t36512 + t36513 - F::new(0.25301106770833333334e-5) * t33228 + t36515 + t36516 - t36517 - t36518;
    (t38708, t38710, t38714)
}
