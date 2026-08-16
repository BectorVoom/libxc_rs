//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2946/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2946(t10263: f64, t13769: f64, t13831: f64, t17800: f64, t2986: f64, t4347: f64, t4518: f64, t4531: f64, t48207: f64, t48210: f64, t48215: f64, t48233: f64, t48242: f64, t48244: f64, t48250: f64, t48256: f64, t5839: f64, t59767: f64, t6733: f64) -> f64 {
    let t61355 = -0.81481481481481481481e-2_f64 * t10263 * t5839 - 0.55555555555555555554e-3_f64 * t48207 + 0.24691358024691358024e-3_f64 * t48210 + 0.57613168724279835389e-3_f64 * t48215 + 0.98765432098765432095e-3_f64 * t48233 + 0.19753086419753086419e-2_f64 * t48242 - 0.13168724279835390946e-2_f64 * t48244 - 0.55555555555555555554e-3_f64 * t2986 * t17800 * t13831 - 0.14814814814814814814e-2_f64 * t2986 * t13769 * t48256 - 0.11111111111111111111e-2_f64 * t2986 * t4531 * t6733 * t4347 - 0.49382716049382716048e-3_f64 * t48250 - 0.11111111111111111111e-2_f64 * t2986 * t4518 * t59767;
    t61355
}
