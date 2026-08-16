//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1082/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1082(t1550: f64, t42970: f64, t44410: f64, t45493: f64, t45495: f64, t45499: f64, t45503: f64, t45505: f64, t45507: f64, t45509: f64, t45514: f64, t45519: f64, t45523: f64, t45525: f64, t45531: f64, t45536: f64, t45541: f64, t530: f64, t6412: f64, t699: f64) -> f64 {
    let t48520 = 0.15323255961587222184e-3_f64 * t45493 - 0.4726e1_f64 * t530 * t44410 + 0.5107751987195740728e-4_f64 * t45495 - 0.212822999466489197e-4_f64 * t45499 - 0.5107751987195740728e-4_f64 * t45503 + t42970 + 0.19863479950205658386e-4_f64 * t45505 - 0.23948483403727617128e0_f64 * t1550 * t699 * t6412 - 0.212822999466489197e-4_f64 * t45507 - 0.212822999466489197e-4_f64 * t45509 - 0.212822999466489197e-4_f64 * t45514 + 0.1702583995731913576e-4_f64 * t45519 - 0.1064114997332445985e-4_f64 * t45523 - 0.5107751987195740728e-4_f64 * t45525 + 0.68186654135613354325e-2_f64 * t45531 + 0.212822999466489197e-4_f64 * t45536 - 0.212822999466489197e-4_f64 * t45541;
    t48520
}
