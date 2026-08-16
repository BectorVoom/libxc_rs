//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 789/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk789(t19135: f64, t28558: f64, t28652: f64, t28660: f64, t31465: f64, t33415: f64, t33436: f64, t33447: f64, t33889: f64, t33894: f64, t33899: f64, t33908: f64, t33912: f64, t33925: f64, t33928: f64, t33934: f64, t33935: f64, t33941: f64, t33942: f64, t33947: f64, t33948: f64) -> f64 {
    let t33951 = -0.20527106943485609994e0_f64 * t19135 * t33889 + 0.18125821328051150223e0_f64 * t28652 * t33894 - 0.18125821328051150223e0_f64 * t28660 * t33899 + t33925 + 0.30209702213418583705e-1_f64 * t28558 * t33415 - 0.45306850413028723348e0_f64 * t33928 * t33908 + 0.22653425206514361674e0_f64 * t31465 * t33912 + 0.80027204934668021496e-1_f64 * t33934 * t33436 * t33935 - 0.12004080740200203224e0_f64 * t33941 * t33436 * t33942 + t33947 + 0.26675734978222673832e-1_f64 * t33948 * t33447;
    t33951
}
