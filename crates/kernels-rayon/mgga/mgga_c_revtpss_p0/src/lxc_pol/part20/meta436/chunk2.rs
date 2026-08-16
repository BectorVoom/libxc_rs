//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1645/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1645(t422: f64, t44999: f64, t45012: f64, t44087: f64, t44096: f64, t44100: f64, t44103: f64, t44106: f64, t44108: f64, t44111: f64, t44114: f64, t44122: f64, t44984: f64, t44987: f64) -> (f64, f64) {
    let t45015 = 0.621814e-1_f64 * (t44999 + t45012) * t422;
    let t45016 = t44087 + t44096 + t44100 - t44103 + t44106 + t44108 - t44111 - t44114 + t44122 + t44984 - t44987 - t45015;
    (t45015, t45016)
}
