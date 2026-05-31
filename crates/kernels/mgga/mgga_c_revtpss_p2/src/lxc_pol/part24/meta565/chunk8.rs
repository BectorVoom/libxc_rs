//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1722/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1722<F: Float>(t6396: F, t6400: F, t1102: F, t198: F, t3336: F, t336: F, t41937: F, t88510: F, t88562: F, t88564: F, t88567: F, t88607: F, t88682: F, t88986: F, t88989: F, t88991: F, t88993: F, t88995: F, t89397: F, t89437: F, t89740: F) -> F {
    let t89746 = t6396 * t6396;
    let t89751 = t6400 * t6400;
    let t89756 = t88510 - t88607 + t198 * t336 * (t88682 + t89397 + t89437 + t89740) * t1102 - t88562 + t88564 - t88567 - F::cast_from(3.0_f64) * t198 * t336 * t89746 * t3336 + t88986 - t88989 + t88991 + t88993 + t88995 - F::cast_from(6.0_f64) * t198 * t336 * t89751 * t41937;
    t89756
}
