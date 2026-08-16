//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1243/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1243(t19731: f64, t550: f64, t1380: f64, t3792: f64, t5286: f64, t5335: f64, t1824: f64, t1834: f64, t5250: f64, t562: f64, t6387: f64, t12250: f64, t1351: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19732 = t19731 * t550;
    let t19733 = t1380 * t19732;
    let t19735 = t3792 * t5286;
    let t19736 = t5335 * t19735;
    let t19739 = t1834 * t1824;
    let t19740 = t19739 * t5250;
    let t19743 = t562 * t6387;
    let t19744 = t12250 * t1351;
    (t19732, t19733, t19735, t19736, t19739, t19740, t19743, t19744)
}
