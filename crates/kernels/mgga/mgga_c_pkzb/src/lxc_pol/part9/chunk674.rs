//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 674/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk674<F: Float>(t3113: F, t378: F, t1201: F, t881: F, t1209: F, t889: F, t2175: F, t2224: F, t2303: F, t2308: F, t3017: F, t3028: F, t3042: F, t3047: F, t3053: F, t3055: F, t3059: F, t3063: F, t3067: F) -> (F, F, F, F) {
    let t3114 = t3113 * t378;
    let t3116 = t1201 * t881;
    let t3121 = t1209 * t889;
    let t3135 = -F::cast_from(0.1294625e1_f64) * t3042 + F::cast_from(0.258925e1_f64) * t3047 + t2303 - F::cast_from(0.301925e0_f64) * t2175 - F::cast_from(0.301925e0_f64) * t3017 + F::cast_from(0.905775e0_f64) * t3028 + F::cast_from(0.82524375e-1_f64) * t3053 + F::cast_from(0.16504875e0_f64) * t3055 + t2308 - F::cast_from(0.16557e0_f64) * t2224 - F::cast_from(0.16557e0_f64) * t3059 + F::cast_from(0.248355e0_f64) * t3063 + F::cast_from(0.248355e0_f64) * t3067;
    (t3114, t3116, t3121, t3135)
}
