//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1852/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1852<F: Float>(t1060: F, t14595: F, t4673: F, t4677: F, t1625: F, t3120: F, t14506: F, t3199: F, t1058: F, t11034: F, t11051: F, t11059: F, t11065: F, t14572: F, t14574: F, t14578: F, t14581: F, t14587: F, t14591: F, t1630: F, t1632: F, t3076: F, t3180: F, t3186: F, t3193: F, t3200: F, t3202: F, t4669: F, t4674: F, t4678: F, t4681: F) -> (F, F, F, F, F, F) {
    let t14596 = t14595 * t1060;
    let t14600 = t4677 * t4673;
    let t14605 = t1625 * t3120;
    let t14606 = t14605 * t1060;
    let t14608 = t14506 * t3199;
    let t14613 = t1058 * t14572 + F::cast_from(2.0_f64) * t1058 * t14587 + t1058 * t14596 + t1058 * t14606 + F::cast_from(4.0_f64) * t11034 * t4674 + t11051 * t1630 + F::cast_from(6.0_f64) * t11059 * t14578 - F::cast_from(6.0_f64) * t11065 * t14591 - F::cast_from(2.0_f64) * t14574 * t3200 + F::cast_from(4.0_f64) * t14581 * t3186 + F::cast_from(4.0_f64) * t14600 * t3186 - t14608 * t3202 + t1632 * t3076 + F::cast_from(2.0_f64) * t3180 * t4678 + F::cast_from(2.0_f64) * t3180 * t4681 + F::cast_from(2.0_f64) * t3193 * t4669;
    (t14596, t14600, t14605, t14606, t14608, t14613)
}
