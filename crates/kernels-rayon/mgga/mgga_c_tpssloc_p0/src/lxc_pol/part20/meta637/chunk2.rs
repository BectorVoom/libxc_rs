//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2344/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2344(t10186: f64, t13799: f64, t13858: f64, t13862: f64, t13865: f64, t13868: f64, t13877: f64, t48052: f64, t48061: f64, t48063: f64, t48067: f64, t48068: f64) -> f64 {
    let t48076 = -0.27777777777777777777e-3_f64 * t48052 - 0.69135802469135802467e-2_f64 * t10186 * t13799 + 0.22222222222222222221e-2_f64 * t10186 * t13858 + 0.44444444444444444442e-2_f64 * t10186 * t13862 - 0.27777777777777777777e-3_f64 * t48061 + 0.44444444444444444443e-2_f64 * t48063 + t48067 + 0.14814814814814814814e-2_f64 * t48068 + 0.88888888888888888885e-2_f64 * t10186 * t13865 + 0.44444444444444444442e-2_f64 * t10186 * t13868 + 0.17777777777777777777e-1_f64 * t10186 * t13877;
    t48076
}
