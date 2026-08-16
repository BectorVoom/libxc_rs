//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 846/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk846(t3124: f64, t406: f64, t3142: f64, t741: f64, t750: f64, t5433: f64, t5437: f64, t5441: f64, t5444: f64, t5451: f64, t5454: f64, t7751: f64, t7753: f64, t7756: f64) -> f64 {
    let t8946 = t406 * t3124;
    let t8948 = t3142 * t741;
    let t8950 = t3142 * t750;
    let t8954 = t5433 - t5437 + t5441 - 4.0_f64 * t8946 - 0.11696447245269292414e1_f64 * t8948 + 0.17315859105681463759e2_f64 * t8950 + t5444 - t7751 - 0.1143056e0_f64 * t7753 + 0.16008171603946666666e-1_f64 * t7756 + t5451 + t5454;
    t8954
}
