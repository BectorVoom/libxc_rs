//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3193/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3193<F: Float>(t1261: F, t24807: F, t3172: F, t17377: F, t20786: F, t1042: F, t12956: F, t17505: F, t17547: F, t17550: F, t20811: F, t20825: F, t20864: F, t20868: F, t24640: F, t3711: F, t5302: F, t5381: F, t5391: F, t6625: F, t82368: F, t82543: F) -> F {
    let t83871 = t1261 * t3172 * t24807;
    let t83891 = t17377 * t20786;
    let t83893 = F::cast_from(0.42874018118069736972e-2_f64) * t5381 * t20868 - F::cast_from(0.22866142996303859718e-2_f64) * t17505 * t20811 - F::cast_from(0.57165357490759649296e-3_f64) * t83871 - F::cast_from(0.7145669686344956162e-3_f64) * t12956 * t24640 - F::cast_from(0.7145669686344956162e-3_f64) * t3711 * t1042 * t5302 * t82368 + F::cast_from(0.3811023832717309953e-2_f64) * t17505 * t20825 - F::cast_from(0.14291339372689912324e-2_f64) * t3711 * t1042 * t17550 * t82543 - F::cast_from(0.7622047665434619906e-2_f64) * t5391 * t20864 - F::cast_from(0.22866142996303859718e-1_f64) * t5391 * t20868 - F::cast_from(0.34299214494455789577e-2_f64) * t17547 * t6625 - F::cast_from(0.42874018118069736972e-3_f64) * t83891;
    t83893
}
