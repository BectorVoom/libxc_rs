//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 941/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk941<F: Float>(t49266: F, t62246: F, t77914: F, t77917: F, t77920: F, t77935: F, t77990: F, t86986: F, t86989: F, t86992: F, t86995: F, t86998: F, t87002: F, t87006: F, t87011: F, t62287: F, t62309: F, t62317: F, t78001: F, t78012: F, t78015: F, t78027: F, t87024: F, t87027: F, t87030: F, t87033: F, t87037: F, t87042: F, t87045: F, t87048: F) -> (F, F) {
    let t87128 = 8.0 * t77914 + 8.0 / 3.0 * t77917 + 40.0 / 81.0 * t77920 - t86986 + 8.0 / 3.0 * t86989 - 8.0 / 9.0 * t86992 + 4.0 / 3.0 * t86995 - 12.0 * t86998 + 6.0 * t87002 - 3.0 / 4.0 * t87006 + 4.0 / 3.0 * t77935 - 8.0 * t87011 - 8.0 / 3.0 * t62246 + 112.0 / 27.0 * t49266 - 8.0 / 3.0 * t77990;
    let t87144 = -8.0 / 9.0 * t78001 + 4.0 / 9.0 * t78012 - 16.0 / 9.0 * t78015 - 8.0 * t78027 - 8.0 / 9.0 * t62287 - 4.0 * t87024 - 4.0 * t87027 - 8.0 / 3.0 * t87030 - 16.0 / 3.0 * t87033 - t87037 - 16.0 / 27.0 * t62309 + 16.0 / 9.0 * t62317 + 8.0 * t87042 + 4.0 / 3.0 * t87045 + 8.0 * t87048;
    (t87128, t87144)
}
