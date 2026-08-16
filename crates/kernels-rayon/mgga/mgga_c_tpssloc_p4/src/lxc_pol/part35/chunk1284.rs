//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1284/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1284(t2693: f64, t7503: f64, t25132: f64, t81882: f64, t7500: f64, t81911: f64, t25064: f64, t81902: f64, t7521: f64, t81632: f64, t22690: f64, t23171: f64, t25319: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87403 = t7503 * t2693;
    let t87405 = t81882 * t25132;
    let t87432 = t81911 * t7500;
    let t87445 = t81902 * t25064;
    let t87635 = t81632 * t7521;
    let t87653 = t23171 * t22690 * t25319;
    (t87403, t87405, t87432, t87445, t87635, t87653)
}
