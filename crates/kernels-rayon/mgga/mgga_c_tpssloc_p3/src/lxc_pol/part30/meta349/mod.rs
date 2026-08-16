//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta349 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1389;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta349(t13969: f64, t4599: f64, t3039: f64, t3069: f64, t4669: f64, t10231: f64, t4338: f64, t973: f64, t4595: f64, t3130: f64, t3048: f64, t4571: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t13970, t13972, t13995, t14000, t14025, t14027, t14049) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1389(t13969, t4599, t3039, t3069, t4669, t10231, t4338, t973, t4595, t3130, t3048, t4571);
    (t13970, t13972, t13995, t14000, t14025, t14027, t14049)
}
