//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3004/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3004<F: Float>(t11941: F, t127: F, t24032: F, t371: F, t15671: F, t20016: F, t1025: F, t24022: F, t1011: F, t11774: F, t16170: F, t1665: F, t1668: F, t19981: F, t19985: F, t24034: F, t3091: F, t3092: F, t372: F, t42328: F, t42996: F, t43161: F, t4915: F, t54404: F, t54648: F, t54687: F, t54733: F, t6266: F, t6339: F, t66395: F, t66689: F, t66822: F, t66860: F, t66943: F, t67215: F, t77541: F, t77545: F) -> F {
    let t79742 = t11941 * t371 * t127 * t24032;
    let t79744 = t15671 * t20016;
    let t79758 = t1025 * t371 * t127 * t24022;
    let t79768 = -F::cast_from(0.76220476654346199061e-3_f64) * t54648 + t66822 / F::new(216.0) + F::cast_from(0.95275595817932748825e-3_f64) * t66860 - F::cast_from(0.19055119163586549765e-3_f64) * t54687 - F::cast_from(0.14291339372689912324e-2_f64) * t11774 * t372 * t16170 * t1668 * t19981 + F::cast_from(0.85748036236139473944e-3_f64) * t42328 * t66689 * t19985 + F::cast_from(0.76220476654346199061e-3_f64) * t54733 - F::cast_from(0.85748036236139473947e-3_f64) * t79742 + F::cast_from(0.85748036236139473947e-3_f64) * t79744 + F::cast_from(0.68598428988911579157e-2_f64) * t43161 * t24034 - F::cast_from(0.68598428988911579157e-2_f64) * t54404 * t6339 - t1011 * t4915 * t77541 / F::new(48.0) - t1011 * t4915 * t77545 / F::new(48.0) - F::cast_from(0.14291339372689912324e-3_f64) * t79758 + F::cast_from(0.34299214494455789578e-2_f64) * t67215 * t1665 + F::cast_from(0.63517063878621832551e-4_f64) * t42996 + F::cast_from(0.42874018118069736972e-3_f64) * t3091 * t3092 * t66395 * t6266 - F::cast_from(0.17149607247227894789e-2_f64) * t66943;
    t79768
}
