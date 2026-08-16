//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2446/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2446(t20234: f64, t43070: f64, t10236: f64, t10186: f64, t13851: f64, t13861: f64, t17804: f64, t17817: f64, t21413: f64, t21430: f64, t2986: f64, t2988: f64, t2990: f64, t341: f64, t43069: f64, t4510: f64, t4518: f64, t4548: f64, t5836: f64, t68534: f64, t68539: f64, t68543: f64, t68547: f64, t69487: f64, t69496: f64, t69503: f64, t69505: f64, t69515: f64) -> f64 {
    let t69519 = t43070 * t20234;
    let t69529 = t10236 * t20234;
    let t69533 = 0.16666666666666666666e-2_f64 * t2986 * t13851 * t17817 - 0.44444444444444444443e-2_f64 * t10186 * t21430 + 0.55555555555555555553e-3_f64 * t69487 - 0.24999999999999999999e-2_f64 * t2986 * t341 * t5836 * t4548 - 0.83333333333333333331e-3_f64 * t2986 * t17804 * t13861 - 0.27777777777777777777e-3_f64 * t2986 * t69496 * t2990 - 0.29629629629629629629e-2_f64 * t10186 * t21413 + 0.37037037037037037037e-3_f64 * t69503 - 0.27777777777777777777e-3_f64 * t2986 * t69505 * t2990 - 0.55555555555555555554e-3_f64 * t2986 * t4518 * t68534 + 0.37037037037037037036e-3_f64 * t2986 * t4510 * t68539 - 0.27777777777777777777e-3_f64 * t2986 * t2988 * t69515 - 0.86419753086419753084e-3_f64 * t2986 * t43069 * t69519 - 0.66666666666666666664e-2_f64 * t2986 * t4518 * t68543 + 0.49999999999999999998e-2_f64 * t2986 * t4518 * t68547 - 0.16666666666666666666e-2_f64 * t2986 * t2988 * t69529;
    t69533
}
