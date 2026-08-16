//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2945/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2945<F: Float>(t10237: F, t2986: F, t340: F, t343: F, t4518: F, t48061: F, t48063: F, t48066: F, t48068: F, t48189: F, t59730: F, t61307: F, t61310: F, t61313: F, t61315: F, t61322: F, t61327: F, t973: F, t974: F) -> F {
    let t61332 = -F::cast_from(0.18518518518518518518e-3_f64) * t48061 + F::cast_from(0.29629629629629629628e-2_f64) * t48063 + F::cast_from(0.74074074074074074072e-3_f64) * t48066 + F::cast_from(0.98765432098765432096e-3_f64) * t48068 + F::cast_from(0.14814814814814814814e-2_f64) * t61307 + F::cast_from(0.18518518518518518518e-3_f64) * t61310 + F::cast_from(0.18518518518518518518e-3_f64) * t61313 - F::cast_from(0.16666666666666666666e-2_f64) * t973 * t974 * t340 * t61315 * t343 - F::cast_from(0.18518518518518518518e-3_f64) * t48189 - F::cast_from(0.37037037037037037036e-3_f64) * t2986 * t61322 * t10237 - F::cast_from(0.18518518518518518518e-3_f64) * t61327 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t4518 * t59730;
    t61332
}
