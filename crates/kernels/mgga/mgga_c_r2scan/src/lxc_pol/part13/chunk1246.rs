//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1246/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1246<F: Float>(t1035: F, t1339: F, t352: F, t1343: F, t3675: F, t11148: F, t11157: F, t11162: F, t11993: F, t31912: F, t31929: F, t3420: F, t37204: F, t37209: F, t37223: F, t37226: F) -> F {
    let t41058 = t1035 * t1339 * t352;
    let t41065 = t3675 * t1343;
    let t41086 = -F::new(0.1575e1) * t3420 * t31929 - F::new(0.354375e1) * t37209 * t41058 - F::new(0.126e2) * t11157 * t11993 - F::new(0.126e2) * t3420 * t31912 - F::new(0.63e1) * t3420 * t41065 - F::new(0.252e2) * t11148 * t41058 - F::new(0.567e2) * t11162 * t41058 - F::new(0.189e2) * t37223 * t11993 - F::new(0.945e1) * t11148 * t41065 - F::new(0.189e2) * t11148 * t31912 - F::new(0.2835e2) * t37226 * t41058 - F::new(0.4725e1) * t37204 * t11993 - F::new(0.4725e1) * t11162 * t31912 - F::new(0.23625e1) * t11162 * t41065;
    t41086
}
