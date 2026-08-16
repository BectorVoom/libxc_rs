//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3085/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3085(t43830: f64, t43832: f64, t44307: f64, t56151: f64, t56155: f64, t56159: f64, t56163: f64, t56167: f64, t56174: f64, t56176: f64, t56181: f64, t56185: f64, t56187: f64, t56189: f64, t56194: f64, t56198: f64, t56203: f64, t56207: f64, t56209: f64, t56447: f64) -> f64 {
    let t56456 = t44307 - 0.19999999999999999999e0_f64 * t56151 + 0.50000000000000000001e-1_f64 * t56155 + 0.15e0_f64 * t56159 + 0.16666666666666666667e-1_f64 * t56163 + 0.2e0_f64 * t56167 - 0.16666666666666666667e-1_f64 * t43830 + 0.55555555555555555557e-2_f64 * t43832 - 0.24691358024691358025e-1_f64 * t56174 - 0.74074074074074074074e-2_f64 * t56176 + 0.11111111111111111111e0_f64 * t56181 + t56447 - 0.33333333333333333333e-1_f64 * t56185 - 0.16666666666666666667e-1_f64 * t56187 - 0.5e-1_f64 * t56189 - 0.16666666666666666666e-1_f64 * t56194 - 0.16666666666666666666e-1_f64 * t56198 - 0.1e0_f64 * t56203 - 0.55555555555555555555e-2_f64 * t56207 + 0.11111111111111111111e-1_f64 * t56209;
    t56456
}
