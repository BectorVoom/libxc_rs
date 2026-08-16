//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1079/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1079(t776: f64, t868: f64, t10189: f64, t344: f64, t134: f64, t2978: f64, t10213: f64, t60: f64, t135: f64, t340: f64, t6733: f64, t884: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13487 = t776 * t868;
    let t13779 = t10189 * t344;
    let t13783 = t134 * t2978;
    let t13784 = t13783 * t344;
    let t13797 = t60 * t10213;
    let t13798 = t13797 * t344;
    let t13822 = t135 * t340;
    let t13831 = t6733 * t884;
    (t13487, t13779, t13783, t13784, t13797, t13798, t13822, t13831)
}
