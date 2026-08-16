//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1189/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1189(t80743: f64, t12022: f64, t12027: f64, t12033: f64, t12444: f64, t1375: f64, t2091: f64, t24082: f64, t3889: f64, t40591: f64, t7194: f64, t7199: f64, t7214: f64, t80722: f64, t80725: f64, t80728: f64, t80735: f64, t80738: f64) -> f64 {
    let t84400 = 0.3244175520728446583e0_f64 * t80743;
    let t84409 = -6.0_f64 * t12444 * t7214 + 0.38381794893125283518e0_f64 * t80722 + 0.24674011002723396548e-1_f64 * t80725 - 0.69087230807625510332e0_f64 * t80728 - 0.39478417604357434476e0_f64 * t80735 - 0.24674011002723396548e-1_f64 * t80738 + 6.0_f64 * t7194 * t12027 - t84400 + 6.0_f64 * t24082 * t3889 + 24.0_f64 * t1375 * t40591 * t2091 * t12022 + 6.0_f64 * t12033 * t7199;
    t84409
}
