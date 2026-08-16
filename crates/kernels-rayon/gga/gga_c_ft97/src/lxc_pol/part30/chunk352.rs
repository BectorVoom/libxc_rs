//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 352/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk352(t238: f64, t1417: f64, t3759: f64, t3766: f64, t3774: f64, t6015: f64, t6019: f64, t6024: f64, t6029: f64, t6034: f64, t6038: f64, t6043: f64, t6047: f64, t6053: f64, t6055: f64, t6057: f64) -> f64 {
    let t239 = 0.1e-59_f64 < t238;
    let t6061 = piecewise3(t239, -0.23254900946437792e-1_f64 * t3759 * t6015 - 2.0_f64 * t3766 * t6019 + 0.25845121844514357744e-4_f64 * t3774 * t6024 + 0.22227677429409423704e-2_f64 * t1417 * t6029 + 0.22270151833971792333e-3_f64 * t6034 * t6038 + 0.38306165027777777778e-1_f64 * t6043 * t6047 - t6053 - 0.6384360837962962963e-2_f64 * t6055 * t6057, 0.0_f64);
    t6061
}
