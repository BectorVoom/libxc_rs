//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3193/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3193(t1261: f64, t24807: f64, t3172: f64, t17377: f64, t20786: f64, t1042: f64, t12956: f64, t17505: f64, t17547: f64, t17550: f64, t20811: f64, t20825: f64, t20864: f64, t20868: f64, t24640: f64, t3711: f64, t5302: f64, t5381: f64, t5391: f64, t6625: f64, t82368: f64, t82543: f64) -> f64 {
    let t83871 = t1261 * t3172 * t24807;
    let t83891 = t17377 * t20786;
    let t83893 = 0.42874018118069736972e-2_f64 * t5381 * t20868 - 0.22866142996303859718e-2_f64 * t17505 * t20811 - 0.57165357490759649296e-3_f64 * t83871 - 0.7145669686344956162e-3_f64 * t12956 * t24640 - 0.7145669686344956162e-3_f64 * t3711 * t1042 * t5302 * t82368 + 0.3811023832717309953e-2_f64 * t17505 * t20825 - 0.14291339372689912324e-2_f64 * t3711 * t1042 * t17550 * t82543 - 0.7622047665434619906e-2_f64 * t5391 * t20864 - 0.22866142996303859718e-1_f64 * t5391 * t20868 - 0.34299214494455789577e-2_f64 * t17547 * t6625 - 0.42874018118069736972e-3_f64 * t83891;
    t83893
}
