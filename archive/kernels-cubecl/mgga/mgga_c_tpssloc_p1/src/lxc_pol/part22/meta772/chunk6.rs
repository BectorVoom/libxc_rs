//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2638/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2638<F: Float>(t6218: F, t6739: F, t15032: F, t1756: F, t19154: F, t19170: F, t19189: F, t19190: F, t19203: F, t22354: F, t22389: F, t3610: F, t3624: F, t470: F, t493: F, t494: F, t5064: F, t5069: F, t5079: F, t53592: F, t6256: F, t6261: F, t65254: F, t66787: F, t73576: F, t73592: F) -> (F, F) {
    let t73755 = t6218 * t6739;
    let t73789 = -F::cast_from(3.0_f64) * t19189 * t22354 * t3624 + F::cast_from(12.0_f64) * t19203 * t3610 * t6256 - F::cast_from(3.0_f64) * t22389 * t3624 * t5079 + t470 * t493 * t73592 + F::cast_from(3.0_f64) * t15032 * t6261 + F::cast_from(3.0_f64) * t1756 * t66787 + F::cast_from(3.0_f64) * t19154 * t53592 + F::cast_from(6.0_f64) * t19170 * t5064 + F::cast_from(3.0_f64) * t19190 * t5064 + t494 * t73576 + F::cast_from(6.0_f64) * t5069 * t65254;
    (t73755, t73789)
}
