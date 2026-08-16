//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1262/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1262(t322: f64, t44642: f64, t11305: f64, t11319: f64, t11993: f64, t12355: f64, t352: f64, t35213: f64, t3556: f64, t38961: f64, t42106: f64, t42128: f64, t42753: f64, t42757: f64, t855: f64) -> f64 {
    let t332 = 0.25e1_f64 < t322;
    let t44814 = piecewise3(t332, t44642, 0.0_f64);
    let t44842 = -0.105e1_f64 * t855 * t44814 * t352 - 0.126e2_f64 * t3556 * t42753 - 0.252e2_f64 * t11305 * t42757 - 0.567e2_f64 * t11319 * t42757 - 0.126e2_f64 * t12355 * t11993 - 0.189e2_f64 * t42128 * t11993 - 0.189e2_f64 * t11305 * t42753 - 0.2835e2_f64 * t38961 * t42757 - 0.63e1_f64 * t3556 * t35213 - 0.945e1_f64 * t11305 * t35213 - 0.4725e1_f64 * t42106 * t11993 - 0.23625e1_f64 * t11319 * t35213 - 0.4725e1_f64 * t11319 * t42753;
    t44842
}
