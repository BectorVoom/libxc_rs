//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1153/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1153(t2441: f64, t3675: f64, t2983: f64, t352: f64, t856: f64, t11148: f64, t11162: f64, t11993: f64, t12009: f64, t3420: f64, t35213: f64, t37209: f64, t37226: f64, t41039: f64, t41047: f64) -> f64 {
    let t42753 = t3675 * t2441;
    let t42757 = t2983 * t856 * t352;
    let t42774 = -0.63e1_f64 * t3420 * t35213 - 0.945e1_f64 * t11148 * t35213 - 0.4725e1_f64 * t41047 * t11993 - 0.23625e1_f64 * t11162 * t35213 - 0.4725e1_f64 * t11162 * t42753 - 0.354375e1_f64 * t37209 * t42757 - 0.126e2_f64 * t3420 * t42753 - 0.252e2_f64 * t11148 * t42757 - 0.567e2_f64 * t11162 * t42757 - 0.126e2_f64 * t12009 * t11993 - 0.189e2_f64 * t41039 * t11993 - 0.189e2_f64 * t11148 * t42753 - 0.2835e2_f64 * t37226 * t42757;
    t42774
}
