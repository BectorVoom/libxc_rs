//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3194/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3194(t24604: f64, t3172: f64, t5384: f64, t12866: f64, t12956: f64, t17794: f64, t1794: f64, t20946: f64, t24248: f64, t24612: f64, t3625: f64, t3626: f64, t372: f64, t5405: f64, t58851: f64, t58883: f64, t58889: f64, t71187: f64, t71192: f64, t71207: f64, t71232: f64) -> f64 {
    let t83897 = t5384 * t3172 * t24604;
    let t83915 = -0.57165357490759649296e-3_f64 * t83897 + 0.85748036236139473944e-3_f64 * t12956 * t24612 + 0.15244095330869239812e-2_f64 * t71187 - 0.14291339372689912324e-3_f64 * t71192 + t58851 + 0.42874018118069736972e-3_f64 * t71207 - 0.14291339372689912324e-2_f64 * t12866 * t372 * t17794 * t1794 * t20946 - t58883 - 0.57165357490759649295e-3_f64 * t71232 + 0.95275595817932748827e-4_f64 * t58889 - 0.14291339372689912324e-3_f64 * t3625 * t3626 * t24248 * t5405;
    t83915
}
