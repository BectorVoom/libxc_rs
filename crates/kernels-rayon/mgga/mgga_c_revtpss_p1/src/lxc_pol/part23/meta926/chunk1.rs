//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3004/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3004(t11941: f64, t127: f64, t24032: f64, t371: f64, t15671: f64, t20016: f64, t1025: f64, t24022: f64, t1011: f64, t11774: f64, t16170: f64, t1665: f64, t1668: f64, t19981: f64, t19985: f64, t24034: f64, t3091: f64, t3092: f64, t372: f64, t42328: f64, t42996: f64, t43161: f64, t4915: f64, t54404: f64, t54648: f64, t54687: f64, t54733: f64, t6266: f64, t6339: f64, t66395: f64, t66689: f64, t66822: f64, t66860: f64, t66943: f64, t67215: f64, t77541: f64, t77545: f64) -> f64 {
    let t79742 = t11941 * t371 * t127 * t24032;
    let t79744 = t15671 * t20016;
    let t79758 = t1025 * t371 * t127 * t24022;
    let t79768 = -0.76220476654346199061e-3_f64 * t54648 + t66822 / 216.0_f64 + 0.95275595817932748825e-3_f64 * t66860 - 0.19055119163586549765e-3_f64 * t54687 - 0.14291339372689912324e-2_f64 * t11774 * t372 * t16170 * t1668 * t19981 + 0.85748036236139473944e-3_f64 * t42328 * t66689 * t19985 + 0.76220476654346199061e-3_f64 * t54733 - 0.85748036236139473947e-3_f64 * t79742 + 0.85748036236139473947e-3_f64 * t79744 + 0.68598428988911579157e-2_f64 * t43161 * t24034 - 0.68598428988911579157e-2_f64 * t54404 * t6339 - t1011 * t4915 * t77541 / 48.0_f64 - t1011 * t4915 * t77545 / 48.0_f64 - 0.14291339372689912324e-3_f64 * t79758 + 0.34299214494455789578e-2_f64 * t67215 * t1665 + 0.63517063878621832551e-4_f64 * t42996 + 0.42874018118069736972e-3_f64 * t3091 * t3092 * t66395 * t6266 - 0.17149607247227894789e-2_f64 * t66943;
    t79768
}
