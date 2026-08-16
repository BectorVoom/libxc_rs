//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta37 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk268;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk269;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta37(t40: f64, t52: f64, t185: f64, t607: f64, t707: f64, t73: f64, t76: f64, t145: f64, t164: f64, t159: f64, t688: f64, t690: f64, t694: f64, t699: f64, zeta_threshold: f64, t167: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t708, t710, t717, t718, t719, t723, t724, t725, t730) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk268(t40, t52, t185, t607, t707, t73, t76, t145, t164, t159, t688, t690, t694, t699, zeta_threshold);
        let t731 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk269(t167);
    (t708, t710, t717, t718, t719, t723, t724, t725, t730, t731)
}
