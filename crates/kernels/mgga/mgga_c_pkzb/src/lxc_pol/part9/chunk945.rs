//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 945/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk945<F: Float>(t12: F, t7335: F, t5528: F, t972: F, t1837: F, t8: F, t1429: F, t652: F, t1643: F, t1646: F, t2732: F, t2735: F, t6771: F, t82: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t7336 = F::cast_from(0.103295e1_f64) * t7335;
    let t7337 = t5528 * t972;
    let t7340 = t1837 * t8;
    let t7345 = t652 * t1429;
    let t7350 = piecewise3::<F>(t84, F::cast_from(0.0_f64), -F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t7337 * t1643 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t7340 * t6771 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2732 * t1646 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7345 + F::cast_from(2.0_f64) * t2735 * t82);
    (t7336, t7337, t7340, t7345, t7350)
}
