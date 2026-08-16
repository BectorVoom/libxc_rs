//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2945/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2945(t10237: f64, t2986: f64, t340: f64, t343: f64, t4518: f64, t48061: f64, t48063: f64, t48066: f64, t48068: f64, t48189: f64, t59730: f64, t61307: f64, t61310: f64, t61313: f64, t61315: f64, t61322: f64, t61327: f64, t973: f64, t974: f64) -> f64 {
    let t61332 = -0.18518518518518518518e-3_f64 * t48061 + 0.29629629629629629628e-2_f64 * t48063 + 0.74074074074074074072e-3_f64 * t48066 + 0.98765432098765432096e-3_f64 * t48068 + 0.14814814814814814814e-2_f64 * t61307 + 0.18518518518518518518e-3_f64 * t61310 + 0.18518518518518518518e-3_f64 * t61313 - 0.16666666666666666666e-2_f64 * t973 * t974 * t340 * t61315 * t343 - 0.18518518518518518518e-3_f64 * t48189 - 0.37037037037037037036e-3_f64 * t2986 * t61322 * t10237 - 0.18518518518518518518e-3_f64 * t61327 - 0.11111111111111111111e-2_f64 * t2986 * t4518 * t59730;
    t61332
}
