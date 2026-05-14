//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 943/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk943<F: Float>(t40530: F, t62364: F, t62410: F, t86942: F, t86946: F, t86950: F, t86954: F, t86958: F, t86962: F, t86966: F, t86970: F, t86975: F, t86979: F, t62822: F, t64491: F, t77914: F, t77917: F, t77920: F, t77935: F, t77990: F, t78001: F, t86989: F, t86992: F, t86995: F, t86998: F, t87002: F, t87011: F) -> (F, F) {
    let t87175 = t62364 + t40530 - 6.0 * t86942 + 4.0 / 3.0 * t86946 - 40.0 / 243.0 * t86950 - 4.0 / 3.0 * t86954 - t86958 / 18.0 + 4.0 / 3.0 * t86962 + t86966 / 3.0 - t86970 / 9.0 + t62410 - 4.0 / 3.0 * t86975 + 4.0 / 9.0 * t86979;
    let t87187 = 4.0 / 3.0 * t77914 + 4.0 / 9.0 * t77917 + 20.0 / 243.0 * t77920 + 4.0 / 9.0 * t86989 - 4.0 / 27.0 * t86992 + 2.0 / 9.0 * t86995 - 2.0 * t86998 + t87002 + 2.0 / 9.0 * t77935 - 4.0 / 3.0 * t87011 - t62822 + t64491 - 4.0 / 9.0 * t77990 - 4.0 / 27.0 * t78001;
    (t87175, t87187)
}
