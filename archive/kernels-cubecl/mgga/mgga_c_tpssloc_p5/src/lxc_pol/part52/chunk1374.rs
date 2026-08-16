//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1374/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1374<F: Float>(t116152: F, t120121: F, t123023: F, t123050: F, t123052: F, t123054: F, t123056: F, t123058: F, t123060: F, t123062: F, t123067: F, t123072: F, t1458: F, t31237: F, t31239: F, t31880: F, t33152: F, t33154: F, t4072: F, t671: F, t8446: F) -> F {
    let t123074 = F::cast_from(2.0_f64) * t116152 * t1458 + F::cast_from(2.0_f64) * t123062 * t671 + F::cast_from(2.0_f64) * t123067 * t1458 + F::cast_from(2.0_f64) * t31880 * t4072 + t120121 + t123023 + F::cast_from(2.0_f64) * t123050 + F::cast_from(2.0_f64) * t123052 + F::cast_from(2.0_f64) * t123054 + F::cast_from(2.0_f64) * t123056 + F::cast_from(2.0_f64) * t123058 + F::cast_from(2.0_f64) * t123060 + F::cast_from(2.0_f64) * t123072 + t31237 + t31239 + t33152 + t33154 + t8446;
    t123074
}
