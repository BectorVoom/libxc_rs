//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1045/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1045(t34804: f64, t34844: f64, t34879: f64, t34897: f64, t35022: f64, t35043: f64, t35055: f64, t35076: f64, t35180: f64, t35204: f64, t35238: f64, t35240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37252 = 0.20965394859736101378e-2_f64 * t34804;
    let t37271 = 0.34299214494455789578e-2_f64 * t34844;
    let t37287 = 0.85748036236139473944e-3_f64 * t34879;
    let t37293 = 0.13073958333333333333e0_f64 * t34897;
    let t37345 = 0.57165357490759649296e-3_f64 * t35022;
    let t37363 = 35.0_f64 / 108.0_f64 * t35043;
    let t37366 = 0.15724046144802076034e-2_f64 * t35055;
    let t37375 = 77.0_f64 / 288.0_f64 * t35076;
    let t37426 = 0.21437009059034868486e-3_f64 * t35180;
    let t37435 = 0.13976929906490734252e-1_f64 * t35204;
    let t37446 = 0.21437009059034868486e-2_f64 * t35238;
    let t37447 = 0.12862205435420921092e-1_f64 * t35240;
    (t37252, t37271, t37287, t37293, t37345, t37363, t37366, t37375, t37426, t37435, t37446, t37447)
}
