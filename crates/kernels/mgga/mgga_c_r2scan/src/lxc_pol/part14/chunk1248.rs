//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1248/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1248<F: Float>(t322: F, t41891: F, t11305: F, t11319: F, t11993: F, t31912: F, t352: F, t3556: F, t38958: F, t38961: F, t38971: F, t38976: F, t41058: F, t41065: F, t855: F) -> F {
    let t332 = F::cast_from(0.25e1_f64) < t322;
    let t42070 = piecewise3::<F>(t332, t41891, F::cast_from(0.0_f64));
    let t42098 = -F::cast_from(0.105e1_f64) * t855 * t42070 * t352 - F::cast_from(0.126e2_f64) * t3556 * t31912 - F::cast_from(0.63e1_f64) * t3556 * t41065 - F::cast_from(0.252e2_f64) * t11305 * t41058 - F::cast_from(0.567e2_f64) * t11319 * t41058 - F::cast_from(0.189e2_f64) * t38958 * t11993 - F::cast_from(0.945e1_f64) * t11305 * t41065 - F::cast_from(0.189e2_f64) * t11305 * t31912 - F::cast_from(0.2835e2_f64) * t38961 * t41058 - F::cast_from(0.4725e1_f64) * t38971 * t11993 - F::cast_from(0.4725e1_f64) * t11319 * t31912 - F::cast_from(0.23625e1_f64) * t11319 * t41065 - F::cast_from(0.354375e1_f64) * t38976 * t41058;
    t42098
}
