//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1076/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1076<F: Float>(t191: F, t192: F, t6872: F, t2020: F, t6876: F, t8494: F, t6997: F, t8450: F, t1393: F, t31062: F, t31065: F, t31067: F, t31070: F, t31072: F, t31077: F, t31078: F, t31080: F, t31082: F, t31088: F, t31089: F, t31223: F, t31224: F, t31240: F, t574: F, t6517: F, t652: F, t6539: F, t672: F, t8447: F) -> (F, F) {
    let t31246 = t6872 * t191 * t192;
    let t31247 = t31246 * t2020;
    let t31249 = t6876 * t8494;
    let t31250 = t8450 * t6997;
    let t31252 = t1393 * t8447 - F::cast_from(2.0_f64) * t31062 * t652 - F::cast_from(2.0_f64) * t31224 * t672 + t31240 * t574 - F::cast_from(4.0_f64) * t6517 * t6539 - F::cast_from(4.0_f64) * t31065 - F::cast_from(4.0_f64) * t31067 - F::cast_from(4.0_f64) * t31070 - F::cast_from(4.0_f64) * t31072 - t31077 - F::cast_from(4.0_f64) * t31078 - F::cast_from(4.0_f64) * t31080 - F::cast_from(4.0_f64) * t31082 - t31088 + t31089 + t31223 + F::cast_from(2.0_f64) * t31247 - t31249 + F::cast_from(2.0_f64) * t31250;
    (t31246, t31252)
}
