//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 999/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk999(t78054: f64, t5148: f64, t551: f64, t71916: f64, t76331: f64, t76333: f64, t2471: f64, t797: f64, t2136: f64, t14509: f64, t8562: f64, t2333: f64, t71887: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t78055 = 0.5987120850931904282e-1_f64 * t78054;
    let t78060 = 0.11974241701863808564e0_f64 * t5148 * t71916 * t551;
    let t78061 = 0.44903406381989282115e-1_f64 * t76331;
    let t78062 = 0.44903406381989282115e-1_f64 * t76333;
    let t78063 = t797 * t2471;
    let t78064 = t78063 * t2136;
    let t78065 = 0.10227998120342003148e-1_f64 * t78064;
    let t78066 = t14509 * t8562;
    let t78067 = 0.13637330827122670864e-1_f64 * t78066;
    let t78068 = t71887 * t2333;
    (t78055, t78060, t78061, t78062, t78065, t78067, t78068)
}
