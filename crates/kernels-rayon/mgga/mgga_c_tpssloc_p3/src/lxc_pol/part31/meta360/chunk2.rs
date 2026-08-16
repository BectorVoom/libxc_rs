//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1280/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1280(t16111: f64, t3739: f64, t12225: f64, t16095: f64, t2586: f64, t1338: f64, t5318: f64, t3866: f64, t5310: f64, t3799: f64, t5289: f64, t2371: f64, t5154: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16113 = 0.16666666666666666666e-2_f64 * t3739 * t16111;
    let t16118 = t12225 * t16095;
    let t16119 = t2586 * t16118;
    let t16132 = t1338 * t5318;
    let t16147 = 35.0_f64 / 576.0_f64 * t3866 * t5310;
    let t16159 = 7.0_f64 / 2304.0_f64 * t3799 * t5289;
    let t16164 = t5154 * t2371;
    (t16113, t16119, t16132, t16147, t16159, t16164)
}
