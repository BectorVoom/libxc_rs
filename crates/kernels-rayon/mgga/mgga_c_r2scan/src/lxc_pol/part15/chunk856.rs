//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 856/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk856(t170: f64, t7760: f64, t1651: f64, t2769: f64, t5411: f64, t5413: f64, t5433: f64, t5437: f64, t5441: f64, t5444: f64, t596: f64, t7745: f64, t7751: f64, t7753: f64, t7756: f64) -> f64 {
    let t7761 = t7760 * t170;
    let t7764 = -0.10843581300301739842e-1_f64 * t7745 + 0.5848223622634646207e0_f64 * t5411 + 0.11696447245269292414e1_f64 * t5413 + t5433 - t5437 + t5441 + t5444 - t7751 - 0.571528e-1_f64 * t7753 + 0.80040858019733333332e-2_f64 * t7756 - 0.675260332e-1_f64 * t1651 * t2769 - 0.1350520664e0_f64 * t596 * t7761;
    t7764
}
