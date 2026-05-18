//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1227/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1227<F: Float>(t17724: F, t2873: F, t730: F, t1987: F, t7536: F, t1999: F, t7560: F, t2860: F, t5494: F, t307: F, t6000: F, t2887: F, t2890: F, t487: F) -> (F, F, F, F, F, F) {
    let t21324 = F::new(0.17315859105681463759e2) * t730 * t2873 * t17724;
    let t21329 = F::new(0.10389515463408878255e3) * t1987 * t7536;
    let t21331 = F::new(0.51947577317044391276e2) * t7560 * t1999;
    let t21333 = F::new(0.10254018858216406658e4) * t2860 * t5494;
    let t21346 = t307 * t6000;
    let t21359 = t2887 * t487 * t2890;
    (t21324, t21329, t21331, t21333, t21346, t21359)
}
