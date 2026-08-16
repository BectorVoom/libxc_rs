//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2624/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2624<F: Float>(t11539: F, t1174: F, t22055: F, t18454: F, t4889: F, t1180: F, t1184: F, t1714: F, t18321: F, t18523: F, t18550: F, t18555: F, t22032: F, t460: F, t4928: F, t4934: F, t4937: F, t6138: F, t73113: F, t73287: F, t73290: F) -> F {
    let t73307 = t1174 * t11539 * t22055;
    let t73314 = t4889 * t18454;
    let t73316 = F::cast_from(0.12674897119341563786e-1_f64) * t73113 * t1180 - F::cast_from(0.24444444444444444444e-1_f64) * t18321 * t4937 - F::cast_from(0.9259259259259259259e-4_f64) * t73287 - F::cast_from(0.8333333333333333333e-3_f64) * t73290 - F::cast_from(0.24999999999999999999e-2_f64) * t1174 * t4934 * t18523 * t1714 * t460 - F::cast_from(0.24999999999999999999e-2_f64) * t1174 * t4934 * t6138 * t4928 * t460 + F::cast_from(0.13333333333333333333e-1_f64) * t4889 * t18550 + F::cast_from(0.66666666666666666666e-2_f64) * t4889 * t18555 + F::cast_from(0.7407407407407407407e-3_f64) * t73307 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t4934 * t22032 * t1184 * t460 + F::cast_from(0.7407407407407407407e-3_f64) * t73314;
    t73316
}
