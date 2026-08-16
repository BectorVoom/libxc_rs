//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1889/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1889(t1081: f64, t4255: f64, t870: f64, t23788: f64, t58071: f64, t86706: f64, t1649: f64, t2745: f64, t25927: f64, t86713: f64, t2379: f64, t2553: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t89859 = t870 * t1081 * t4255;
    let t89862 = t23788 * t58071;
    let t89865 = t23788 * t86706;
    let t89868 = t1649 * t2745;
    let t89872 = t25927 * t86713;
    let t89874 = t1649 * t2379;
    let t89881 = t1649 * t2553;
    (t89859, t89862, t89865, t89868, t89872, t89874, t89881)
}
