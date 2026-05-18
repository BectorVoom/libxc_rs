//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1289/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1289<F: Float>(t2240: F, t3069: F, t9859: F, t11243: F, t6199: F, t851: F, t10000: F, t10003: F, t11180: F, t11214: F, t11217: F, t18513: F, t18866: F, t18878: F, t2296: F, t2318: F, t3135: F, t31357: F, t31369: F, t31372: F, t31375: F, t31377: F, t31380: F, t6282: F, t8120: F, t8211: F, t889: F, t9985: F, t9992: F) -> (F, F, F) {
    let t31383 = F::new(0.48245938496077605201e2) * t2240 * t9859 * t3069;
    let t31390 = F::new(0.57895126195293126241e3) * t6199 * t11243 * t851;
    let t31391 = F::new(0.51947577317044391277e2) * t2318 * t9985 * t3135 - F::new(0.12304822629859687989e5) * t18866 * t11217 * t889 - F::new(0.11696447245269292414e1) * t2296 * t11214 * t889 + F::new(0.17315859105681463759e2) * t2318 * t31357 * t889 + F::new(0.30762056574649219974e4) * t6282 * t9992 * t3135 + F::new(0.91082604192152556044e5) * t18878 * t11180 * t18513 * t889 + t31369 + t31372 + t31375 - t31377 - t31380 - t31383 + F::new(18.0) * t8120 * t10000 - F::new(12.0) * t8211 * t10003 - t31390;
    (t31383, t31390, t31391)
}
