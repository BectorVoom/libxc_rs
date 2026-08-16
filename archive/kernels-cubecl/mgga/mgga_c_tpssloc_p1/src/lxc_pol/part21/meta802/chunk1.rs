//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2790/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2790<F: Float>(t41274: F, t39658: F, t41254: F, t41258: F, t41262: F, t58983: F, t58985: F, t58986: F, t58987: F, t58988: F, t58989: F, t58990: F, t58991: F, t58993: F, t58996: F, t58999: F, t59001: F, t59005: F, t59008: F) -> (F, F) {
    let t59009 = F::cast_from(0.11696447245269292414e1_f64) * t41274;
    let t59010 = t41254 - t58983 + t58985 - t58986 - t41258 - t58987 - t41262 - t58988 + t58989 + t58990 + t58991 + t58993 + t58996 + t58999 + t59001 + t59005 + t59008 - t39658 + t59009;
    (t59009, t59010)
}
