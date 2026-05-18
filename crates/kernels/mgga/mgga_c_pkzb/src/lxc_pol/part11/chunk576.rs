//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 576/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk576<F: Float>(t1209: F, t889: F, t2175: F, t2224: F, t2303: F, t2308: F, t3017: F, t3028: F, t3042: F, t3047: F, t3053: F, t3055: F, t3059: F, t3063: F, t3067: F) -> (F, F) {
    let t3121 = t1209 * t889;
    let t3135 = -F::new(0.1294625e1) * t3042 + F::new(0.258925e1) * t3047 + t2303 - F::new(0.301925e0) * t2175 - F::new(0.301925e0) * t3017 + F::new(0.905775e0) * t3028 + F::new(0.82524375e-1) * t3053 + F::new(0.16504875e0) * t3055 + t2308 - F::new(0.16557e0) * t2224 - F::new(0.16557e0) * t3059 + F::new(0.248355e0) * t3063 + F::new(0.248355e0) * t3067;
    (t3121, t3135)
}
