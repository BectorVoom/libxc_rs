//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1182/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1182(t34879: f64, t34893: f64, t34895: f64, t34897: f64, t30782: f64, t34883: f64, t34887: f64, t34891: f64, t34901: f64, t34905: f64, t34909: f64, t34913: f64, t34916: f64, t34920: f64, t34923: f64, t34926: f64, t34929: f64, t34933: f64) -> f64 {
    let t37287 = 0.85748036236139473944e-3_f64 * t34879;
    let t37291 = 0.3361875e0_f64 * t34893;
    let t37292 = 0.3361875e0_f64 * t34895;
    let t37293 = 0.13073958333333333333e0_f64 * t34897;
    let t37305 = t37287 - 0.4584375e-1_f64 * t34883 - 0.916875e-1_f64 * t34887 - 0.4584375e-1_f64 * t34891 + t37291 + t37292 - t37293 + 0.4584375e0_f64 * t34901 - t34905 / 8.0_f64 - 0.183375e0_f64 * t34909 - 0.916875e-1_f64 * t34913 - 0.916875e-1_f64 * t34916 - 0.916875e-1_f64 * t34920 - 0.916875e-1_f64 * t34923 - 0.183375e0_f64 * t34926 - 0.916875e-1_f64 * t34929 - 0.183375e0_f64 * t30782 + 0.85748036236139473944e-3_f64 * t34933;
    t37305
}
