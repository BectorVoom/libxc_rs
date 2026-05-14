//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1099/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1099<F: Float>(t322: F, t41891: F, t11305: F, t11319: F, t11993: F, t31912: F, t352: F, t3556: F, t38958: F, t38961: F, t38971: F, t38976: F, t41058: F, t41065: F, t855: F, t12351: F, t1348: F) -> (F, F) {
    let t332 = 0.25e1 < t322;
    let t42070 = piecewise3(t332, t41891, 0.0);
    let t42098 = -0.105e1 * t855 * t42070 * t352 - 0.126e2 * t3556 * t31912 - 0.63e1 * t3556 * t41065 - 0.252e2 * t11305 * t41058 - 0.567e2 * t11319 * t41058 - 0.189e2 * t38958 * t11993 - 0.945e1 * t11305 * t41065 - 0.189e2 * t11305 * t31912 - 0.2835e2 * t38961 * t41058 - 0.4725e1 * t38971 * t11993 - 0.4725e1 * t11319 * t31912 - 0.23625e1 * t11319 * t41065 - 0.354375e1 * t38976 * t41058;
    let t42101 = t1348 * t12351;
    (t42098, t42101)
}
