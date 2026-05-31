//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 903/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk903<F: Float>(t12: F, t4872: F, t1634: F, t192: F, t5093: F, t972: F, t1642: F, t8: F, t1429: F, t439: F, t1643: F, t1646: F, t2540: F, t2543: F, t82: F, t87: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t6762 = F::cast_from(0.21687162600603479684e-1_f64) * t4872;
    let t6763 = t1634 * t192;
    let t6767 = t5093 * t972;
    let t6770 = t1642 * t8;
    let t6771 = t1429 * t439;
    let t6781 = piecewise3::<F>(t84, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t6767 * t1643 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t6770 * t6771 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2540 * t1646 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t87 * t1429 - F::cast_from(8.0_f64) * t2543 * t82);
    (t6762, t6763, t6767, t6770, t6771, t6781)
}
