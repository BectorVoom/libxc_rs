//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta241 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1056;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1057;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1058;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta241(t6581: f64, t805: f64, t1891: f64, t2230: f64, t213: f64, t1895: f64, t202: f64, t243: f64, t598: f64, t1894: f64, t236: f64, t776: f64, t2229: f64, t61: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6582, t6584, t6586, t6589) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1056(t6581, t805, t1891, t2230, t213, t1895, t202, t243);
        let (t6590, t6591) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1057(t598, t6589, t213);
        let (t6593, t6594, t6597) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1058(t1894, t236, t776, t6591, t2229, t61);
    (t6582, t6584, t6586, t6589, t6590, t6591, t6593, t6594, t6597)
}
