//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2344/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2344<F: Float>(t10186: F, t13799: F, t13858: F, t13862: F, t13865: F, t13868: F, t13877: F, t48052: F, t48061: F, t48063: F, t48067: F, t48068: F) -> F {
    let t48076 = -F::cast_from(0.27777777777777777777e-3_f64) * t48052 - F::cast_from(0.69135802469135802467e-2_f64) * t10186 * t13799 + F::cast_from(0.22222222222222222221e-2_f64) * t10186 * t13858 + F::cast_from(0.44444444444444444442e-2_f64) * t10186 * t13862 - F::cast_from(0.27777777777777777777e-3_f64) * t48061 + F::cast_from(0.44444444444444444443e-2_f64) * t48063 + t48067 + F::cast_from(0.14814814814814814814e-2_f64) * t48068 + F::cast_from(0.88888888888888888885e-2_f64) * t10186 * t13865 + F::cast_from(0.44444444444444444442e-2_f64) * t10186 * t13868 + F::cast_from(0.17777777777777777777e-1_f64) * t10186 * t13877;
    t48076
}
