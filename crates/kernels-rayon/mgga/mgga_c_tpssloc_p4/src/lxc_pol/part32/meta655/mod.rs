//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta655 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2084;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta655(t22643: f64, t7691: f64, t81195: f64, t26502: f64, t532: f64, t22573: f64, t7684: f64, t2018: f64, t40611: f64, t86586: f64, t86870: f64, t86911: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t91548, t91620, t91655, t91686, t92121, t92383, t92402) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2084(t22643, t7691, t81195, t26502, t532, t22573, t7684, t2018, t40611, t86586, t86870, t86911);
    (t91548, t91620, t91655, t91686, t92121, t92383, t92402)
}
