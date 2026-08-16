//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1820/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1820(t24649: f64, t7324: f64, t3493: f64, t475: f64, t68: f64, t7328: f64, t2131: f64, t23508: f64, t7325: f64, t3030: f64, t3502: f64, t478: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24650 = t7324 * t24649;
    let t24654 = t3493 * t68 * t475;
    let t24655 = t7328 * t24654;
    let t24658 = t2131 * t23508;
    let t24659 = t24658 * t7325;
    let t24660 = t3030 * t3502;
    let t24661 = t24660 * t478;
    (t24650, t24654, t24655, t24658, t24659, t24660, t24661)
}
