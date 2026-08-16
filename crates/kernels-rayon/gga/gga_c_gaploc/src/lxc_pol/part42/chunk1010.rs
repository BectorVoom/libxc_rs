//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 1010/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk1010(t224: f64, t50308: f64, t50312: f64, t50478: f64, t50800: f64, t14443: f64, t45164: f64, t45973: f64, t45974: f64, t45992: f64, t46006: f64, t46023: f64, t46025: f64, t46835: f64, t49977: f64, t49980: f64, t49983: f64, t50475: f64, t50781: f64, t50789: f64, t50791: f64, t50796: f64, t50799: f64, t617: f64) -> f64 {
    let t50803 = t224 * (t50308 + t50312 + t50478 + t50800);
    let t50805 = t14443 * t617 - t45164 + t45973 - t45974 + t45992 + t46006 + t46023 + t46025 + t46835 - t49977 + t49980 - t49983 + t50475 - t50781 + t50789 - t50791 - t50796 + t50799 + t50803;
    t50805
}
