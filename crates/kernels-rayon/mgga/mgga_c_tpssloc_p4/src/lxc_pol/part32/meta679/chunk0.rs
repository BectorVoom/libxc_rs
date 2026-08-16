//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2118/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2118(t225: f64, t27424: f64, t27422: f64, t24574: f64, t27752: f64, t27834: f64, t3640: f64, t11947: f64, t8090: f64, t27331: f64, t9231: f64, t46104: f64, t7245: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95899 = t27424 * t225;
    let t95902 = t27422 * t225;
    let t95912 = 0.54831135561607547884e-2_f64 * t24574 * t27752;
    let t95921 = t27834 * t3640;
    let t95925 = t8090 * t11947;
    let t95981 = t9231 * t27331;
    let t96025 = t46104 * t7245;
    (t95899, t95902, t95912, t95921, t95925, t95981, t96025)
}
