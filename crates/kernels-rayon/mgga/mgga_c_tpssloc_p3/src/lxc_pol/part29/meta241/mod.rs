//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta241 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1132;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1133;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1134;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1135;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1136;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta241(t214: f64, t252: f64, t225: f64, t258: f64, t776: f64, t6552: f64, t154: f64, t16: f64, t117: f64, t206: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6553 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1132(t214, t252);
        let t6554 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1133(t225, t258);
        let t6555 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1134(t6554, t776);
        let (t6556, t6557, t6559) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1135(t6553, t6555, t6552, t154, t16);
        let (t6561, t6562) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1136(t117, t206, t67, t6559);
    (t6553, t6554, t6555, t6556, t6557, t6559, t6561, t6562)
}
