//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1262/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1262<F: Float>(t322: F, t44642: F, t11305: F, t11319: F, t11993: F, t12355: F, t352: F, t35213: F, t3556: F, t38961: F, t42106: F, t42128: F, t42753: F, t42757: F, t855: F) -> F {
    let t332 = F::new(0.25e1) < t322;
    let t44814 = piecewise3::<F>(t332, t44642, F::new(0.0));
    let t44842 = -F::new(0.105e1) * t855 * t44814 * t352 - F::new(0.126e2) * t3556 * t42753 - F::new(0.252e2) * t11305 * t42757 - F::new(0.567e2) * t11319 * t42757 - F::new(0.126e2) * t12355 * t11993 - F::new(0.189e2) * t42128 * t11993 - F::new(0.189e2) * t11305 * t42753 - F::new(0.2835e2) * t38961 * t42757 - F::new(0.63e1) * t3556 * t35213 - F::new(0.945e1) * t11305 * t35213 - F::new(0.4725e1) * t42106 * t11993 - F::new(0.23625e1) * t11319 * t35213 - F::new(0.4725e1) * t11319 * t42753;
    t44842
}
