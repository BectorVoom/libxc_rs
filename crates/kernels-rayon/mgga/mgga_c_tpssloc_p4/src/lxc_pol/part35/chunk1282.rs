//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1282/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1282(t2627: f64, t7510: f64, t23030: f64, t25258: f64, t7524: f64, t81612: f64, t81613: f64, t23145: f64, t4166: f64, t25132: f64, t81876: f64, t23047: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87142 = t2627 * t7510;
    let t87155 = t23030 * t25258;
    let t87177 = t81612 * t81613 * t7524;
    let t87199 = t4166 * t23145;
    let t87213 = t81876 * t25132;
    let t87218 = t4166 * t23047;
    (t87142, t87155, t87177, t87199, t87213, t87218)
}
