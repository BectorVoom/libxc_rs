//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1256/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1256(t2860: f64, t9348: f64, t30542: f64, t713: f64, t722: f64, t730: f64, t9245: f64, t30758: f64, t30761: f64, t30764: f64, t30767: f64, t30769: f64, t30772: f64, t30775: f64, t30778: f64) -> (f64, f64, f64, f64) {
    let t30780 = 0.70178683471615754484e1_f64 * t2860 * t9348;
    let t30784 = 0.5848223622634646207e0_f64 * t730 * t713 * t30542 * t722;
    let t30786 = 0.10526802520742363173e2_f64 * t2860 * t9245;
    let t30787 = -t30758 + t30761 + t30764 - t30767 + t30769 - t30772 - t30775 + t30778 + t30780 - t30784 - t30786;
    (t30780, t30784, t30786, t30787)
}
