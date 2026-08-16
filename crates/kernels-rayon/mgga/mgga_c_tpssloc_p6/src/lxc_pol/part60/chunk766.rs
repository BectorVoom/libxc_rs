//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 766/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk766(t26246: f64, t26268: f64, t27012: f64, t27019: f64, t27022: f64, t27027: f64, t28058: f64, t28061: f64, t28063: f64, t28065: f64, t28068: f64, t28070: f64, t28074: f64, t28078: f64, t28080: f64) -> f64 {
    let t28083 = t27012 + 0.6728792682356731809e-4_f64 * t26246 - t27019 + 0.40372756094140390854e-3_f64 * t28058 - 0.20186378047070195427e-3_f64 * t28061 - t28063 / 1536.0_f64 - t28065 / 768.0_f64 - 0.20186378047070195427e-3_f64 * t28068 + t27022 + t28070 / 16.0_f64 + 0.84782787797694820792e-2_f64 * t28074 - 0.12111826828242117256e-2_f64 * t28078 - t28080 / 48.0_f64 + t27027 + 0.16956557559538964159e-1_f64 * t26268;
    t28083
}
