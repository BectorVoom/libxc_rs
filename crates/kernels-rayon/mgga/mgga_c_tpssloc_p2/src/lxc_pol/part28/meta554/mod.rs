//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta554 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1825;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta554(t225: f64, t24141: f64, t81072: f64, t81074: f64, t80825: f64, t80847: f64, t80885: f64, t80899: f64, t80956: f64, t80970: f64, t1338: f64, t24063: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t84433, t84480, t84481, t84514, t84520, t84533, t84536, t84555, t84558, t84581) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1825(t225, t24141, t81072, t81074, t80825, t80847, t80885, t80899, t80956, t80970, t1338, t24063);
    (t84433, t84480, t84481, t84514, t84520, t84533, t84536, t84555, t84558, t84581)
}
