//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 459/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk459(t2176: f64, t323: f64, t1968: f64, t1970: f64, t1986: f64, t1989: f64, t1995: f64, t1999: f64, t2010: f64, t2013: f64, t2017: f64, t2021: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2178 = 0.65854491829355115987e0_f64 * t2176 * t323;
    let t2179 = 0.18868855373762491241e-2_f64 * t1968;
    let t2180 = 0.12862205435420921092e-2_f64 * t1970;
    let t2182 = 0.14291339372689912324e-3_f64 * t1986;
    let t2183 = 0.31448092289604152069e-3_f64 * t1989;
    let t2184 = 0.20965394859736101379e-3_f64 * t1995;
    let t2185 = 0.85748036236139473944e-3_f64 * t1999;
    let t2189 = 0.40015750243531754507e-2_f64 * t2010;
    let t2190 = 0.85748036236139473944e-3_f64 * t2013;
    let t2191 = 0.28015625e-1_f64 * t2017;
    let t2192 = 7.0_f64 / 144.0_f64 * t2021;
    (t2178, t2179, t2180, t2182, t2183, t2184, t2185, t2189, t2190, t2191, t2192)
}
