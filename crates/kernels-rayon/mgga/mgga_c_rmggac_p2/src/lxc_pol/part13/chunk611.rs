//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 611/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk611(t7240: f64, t7252: f64, t7256: f64, t7260: f64, t2212: f64, t4965: f64, t2265: f64, t931: f64, t7266: f64, t7276: f64, t7285: f64, t4616: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8025 = 0.638468998399467591e-4_f64 * t7240;
    let t8027 = 0.5107751987195740728e-4_f64 * t7252;
    let t8028 = 0.1702583995731913576e-4_f64 * t7256;
    let t8029 = 0.85129199786595678799e-5_f64 * t7260;
    let t8030 = t4965 * t2212;
    let t8031 = 0.79828278012425390428e-1_f64 * t8030;
    let t8033 = t931 * t2265;
    let t8034 = 0.2363e1_f64 * t8033;
    let t8035 = 0.13637330827122670865e-1_f64 * t7266;
    let t8037 = 0.13637330827122670865e0_f64 * t7276;
    let t8039 = 0.40911992481368012596e-1_f64 * t7285;
    let t8041 = t4616 * t698;
    (t8025, t8027, t8028, t8029, t8031, t8034, t8035, t8037, t8039, t8041)
}
