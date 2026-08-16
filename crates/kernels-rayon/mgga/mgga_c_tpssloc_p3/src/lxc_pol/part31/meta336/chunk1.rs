//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1240/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1240(t13783: f64, t344: f64, t4338: f64, t2986: f64, t10190: f64, t4514: f64, t10213: f64, t60: f64, t135: f64, t340: f64, t4548: f64, t973: f64) -> (f64, f64, f64, f64) {
    let t13784 = t13783 * t344;
    let t13785 = t13784 * t4338;
    let t13787 = 0.24691358024691358024e-3_f64 * t2986 * t13785;
    let t13788 = t10190 * t4514;
    let t13790 = 0.18518518518518518518e-3_f64 * t2986 * t13788;
    let t13797 = t60 * t10213;
    let t13798 = t13797 * t344;
    let t13822 = t135 * t340;
    let t13823 = t13822 * t4548;
    let t13825 = 0.55555555555555555554e-3_f64 * t973 * t13823;
    (t13787, t13790, t13798, t13825)
}
