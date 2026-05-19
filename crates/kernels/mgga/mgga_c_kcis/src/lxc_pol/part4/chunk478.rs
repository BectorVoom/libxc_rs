//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 478/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk478<F: Float>(t1430: F, t1889: F, t1437: F, t1451: F, t104: F, t111: F, t120: F, t1404: F, t1424: F, t1429: F, t1436: F, t1442: F, t1445: F, t1450: F, t1650: F) -> (F, F, F, F) {
    let t1968 = t1430 * t1889;
    let t1971 = t1437 * t1889;
    let t1976 = t1451 * t1889;
    let t1979 = t1424 + F::cast_from(0.11955719325063177623e-1_f64) * t1404 * t1650 - t1429 - F::new(0.3513e-2) * t104 * t1968 + t1436 + F::new(0.7925e-3) * t111 * t1971 - t1442 - F::cast_from(0.5179538907796306876e-4_f64) * t1445 * t1650 + t1450 + F::new(0.50413125e-5) * t120 * t1976;
    (t1968, t1971, t1976, t1979)
}
