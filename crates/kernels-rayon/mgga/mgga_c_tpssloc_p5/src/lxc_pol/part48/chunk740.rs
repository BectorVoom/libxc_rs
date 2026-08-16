//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 740/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk740(t23110: f64, t6648: f64, t23185: f64, t226: f64, t23026: f64, t23029: f64, t23032: f64, t23038: f64, t23151: f64, t23156: f64, t23160: f64, t23167: f64, t23170: f64, t23174: f64, t23178: f64, t23182: f64) -> (f64, f64) {
    let t23186 = t23110 * t6648;
    let t23187 = t23185 * t23186;
    let t23189 = -0.82246703342411321824e-2_f64 * t23026 - t23029 + t23032 + 0.49348022005446793095e-1_f64 * t23038 + t226 * t23151 - 0.3289868133696452873e-1_f64 * t23156 - 0.16449340668482264365e-1_f64 * t23160 + t23167 + t23170 - t23174 - 0.16449340668482264365e-1_f64 * t23178 - 0.82246703342411321825e-2_f64 * t23182 + 0.82246703342411321824e-2_f64 * t23187;
    (t23187, t23189)
}
