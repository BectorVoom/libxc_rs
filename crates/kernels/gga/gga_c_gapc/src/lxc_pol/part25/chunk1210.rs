//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1210/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1210<F: Float>(t33212: F, t33217: F, t33228: F, t36508: F, t36510: F, t36512: F, t36513: F, t36515: F, t36516: F, t36517: F, t36518: F, t33261: F, t36520: F, t36521: F, t36522: F, t36523: F, t36524: F, t36526: F, t36527: F, t36528: F, t36529: F, t36530: F) -> (F, F) {
    let t38714 = t36508 + 0.36231816839129402172e-6 * t33212 - t36510 + 0.18115908419564701086e-6 * t33217 + t36512 + t36513 - 0.25301106770833333334e-5 * t33228 + t36515 + t36516 - t36517 - t36518;
    let t38716 = -t36520 + t36521 + t36522 - t36523 - t36524 + 0.97817934710145362364e-6 * t33261 + t36526 + t36527 + t36528 + t36529 + t36530;
    (t38714, t38716)
}
