//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3194/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3194<F: Float>(t24604: F, t3172: F, t5384: F, t12866: F, t12956: F, t17794: F, t1794: F, t20946: F, t24248: F, t24612: F, t3625: F, t3626: F, t372: F, t5405: F, t58851: F, t58883: F, t58889: F, t71187: F, t71192: F, t71207: F, t71232: F) -> F {
    let t83897 = t5384 * t3172 * t24604;
    let t83915 = -F::cast_from(0.57165357490759649296e-3_f64) * t83897 + F::cast_from(0.85748036236139473944e-3_f64) * t12956 * t24612 + F::cast_from(0.15244095330869239812e-2_f64) * t71187 - F::cast_from(0.14291339372689912324e-3_f64) * t71192 + t58851 + F::cast_from(0.42874018118069736972e-3_f64) * t71207 - F::cast_from(0.14291339372689912324e-2_f64) * t12866 * t372 * t17794 * t1794 * t20946 - t58883 - F::cast_from(0.57165357490759649295e-3_f64) * t71232 + F::cast_from(0.95275595817932748827e-4_f64) * t58889 - F::cast_from(0.14291339372689912324e-3_f64) * t3625 * t3626 * t24248 * t5405;
    t83915
}
