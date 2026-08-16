//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2322/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2322(t1193: f64, t29585: f64, t2136: f64, t29562: f64, t52: f64, t27674: f64, t5040: f64, t1409: f64, t8027: f64, t18368: f64, t27629: f64, t27692: f64, t4954: f64, t8040: f64, t86324: f64, t95384: f64, t95404: f64, t95410: f64, t95424: f64, t95435: f64, t95566: f64, t95678: f64) -> f64 {
    let t104139 = t29585 * t1193;
    let t104142 = t29562 * t52 * t2136;
    let t104150 = t27674 * t5040;
    let t104153 = t8027 * t1409 * t2136;
    let t104155 = t95566 * t4954 / 216.0_f64 - t86324 * t18368 / 1152.0_f64 + t95404 + 11.0_f64 / 324.0_f64 * t104139 + 0.72670960969452703541e-2_f64 * t104142 - 0.20186378047070195428e-3_f64 * t95678 * t8040 - 0.20186378047070195428e-3_f64 * t27629 * t27692 + 0.20186378047070195428e-3_f64 * t95384 * t8040 + t104150 / 162.0_f64 - t95410 - t95424 - t95435 + 0.16149102437656156342e-2_f64 * t104153;
    t104155
}
