//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1193/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1193(t20378: f64, t16822: f64, t16825: f64, t16946: f64, t16950: f64, t20365: f64, t20373: f64, t20377: f64, t29139: f64, t29140: f64, t29141: f64, t29142: f64, t29143: f64, t29145: f64, t29146: f64, t29149: f64, t29150: f64) -> (f64, f64) {
    let t29151 = 180.0_f64 * t20378;
    let t29152 = -t16822 - t20365 - t29139 - t29140 - t29141 + t29142 + t20373 - t29143 + t29145 + t16825 - t29146 - t20377 + t16946 + t16950 - t29149 - t29150 + t29151;
    (t29151, t29152)
}
