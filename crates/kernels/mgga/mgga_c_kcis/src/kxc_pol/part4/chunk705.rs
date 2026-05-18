//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 705/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk705<F: Float>(t1430: F, t3801: F, t1437: F, t1451: F, t104: F, t111: F, t120: F, t2642: F, t4023: F, t4073: F, t4075: F, t4078: F, t4081: F, t4083: F, t4086: F, t4089: F, t4093: F) -> (F, F, F, F) {
    let t4096 = t1430 * t3801;
    let t4099 = t1437 * t3801;
    let t4102 = t1451 * t3801;
    let t4105 = -F::new(0.23526125e-4) * t4073 + F::new(0.50413125e-5) * t120 * t4075 - F::new(0.672175e-5) * t120 * t4078 + F::new(0.9368e-2) * t4081 - F::new(0.3513e-2) * t104 * t4083 + F::new(0.1171e-2) * t104 * t4086 - F::new(0.26416666666666666666e-2) * t4089 - F::new(0.23911438650126355246e-1) * t4023 * t2642 + F::new(0.15538616723388920628e-3) * t4093 * t2642 + F::new(0.7026e-2) * t104 * t4096 - F::new(0.1585e-2) * t111 * t4099 - F::new(0.10082625e-4) * t120 * t4102;
    (t4096, t4099, t4102, t4105)
}
