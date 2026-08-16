//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2790/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2790(t41274: f64, t39658: f64, t41254: f64, t41258: f64, t41262: f64, t58983: f64, t58985: f64, t58986: f64, t58987: f64, t58988: f64, t58989: f64, t58990: f64, t58991: f64, t58993: f64, t58996: f64, t58999: f64, t59001: f64, t59005: f64, t59008: f64) -> (f64, f64) {
    let t59009 = 0.11696447245269292414e1_f64 * t41274;
    let t59010 = t41254 - t58983 + t58985 - t58986 - t41258 - t58987 - t41262 - t58988 + t58989 + t58990 + t58991 + t58993 + t58996 + t58999 + t59001 + t59005 + t59008 - t39658 + t59009;
    (t59009, t59010)
}
