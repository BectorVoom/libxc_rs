//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2195/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2195(t23363: f64, t23382: f64, t868: f64, t225: f64, t23359: f64, t10501: f64, t10503: f64, t10984: f64, t14474: f64, t14486: f64, t14998: f64, t15004: f64, t15006: f64, t15015: f64, t18318: f64, t213: f64, t257: f64, t4474: f64, t6049: f64, t6072: f64, t865: f64) -> (f64, f64, f64, f64) {
    let t23383 = t23363 + t23382;
    let t23384 = t868 * t23383;
    let t23388 = t23359 * t225;
    let t23400 = -0.19514881078765566038e-2_f64 * t14474 + 0.39029762157531132076e-1_f64 * t14486 - 0.65854491829355115987e0_f64 * t865 * t23384 - 0.16463622957338778996e-1_f64 * t18318 + 0.65854491829355115987e0_f64 * t213 * t23388 * t257 + t10501 - 0.21951497276451705329e-1_f64 * t14998 - t10503 - 0.19756347548806534796e1_f64 * t4474 * t6072 + 0.39512695097613069591e1_f64 * t4474 * t6049 - 0.34697458558045176417e-2_f64 * t15004 + t10984 - 0.39029762157531132076e-1_f64 * t15006 + 0.19514881078765566038e-2_f64 * t15015;
    (t23383, t23384, t23388, t23400)
}
