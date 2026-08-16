//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 982/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk982(t2226: f64, t32063: f64, t2131: f64, t2132: f64, t309: f64, t8301: f64, t2217: f64, t7885: f64, t864: f64, t29976: f64, t8337: f64, t8004: f64, t8107: f64) -> (f64, f64, f64, f64, f64) {
    let t33110 = t32063 * t2226;
    let t33114 = t2131 * t2132 * t8301 * t309;
    let t33118 = t7885 * t2132 * t2217 * t864;
    let t33120 = t29976 * t8337;
    let t33124 = t2131 * t8004 * t8107 * t309;
    (t33110, t33114, t33118, t33120, t33124)
}
