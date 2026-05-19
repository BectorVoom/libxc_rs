//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1134/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1134<F: Float>(t12: F, t439: F, t82: F, t1429: F, t1646: F, t2543: F, t500: F, t16232: F, t1642: F, t19633: F, t19636: F, t2540: F, t5093: F, t5094: F, t5100: F, t6767: F, t6770: F, t8: F, t87: F, t972: F, zeta_threshold: F) -> (F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t19642 = t82 * t439;
    let t19645 = t1429 * t1646;
    let t19653 = F::new(32.0) * t2543 * t500;
    let t19655 = piecewise3::<F>(t84, F::new(0.0), F::new(40.0) / F::new(81.0) * t16232 * t972 * t5094 - F::new(16.0) / F::new(9.0) * t5093 * t8 * t19633 - F::new(8.0) / F::new(9.0) * t6767 * t19636 + F::new(8.0) / F::new(3.0) * t1642 * t1429 * t439 - F::new(8.0) * t6770 * t19642 + F::new(8.0) / F::new(3.0) * t6770 * t19645 + F::new(4.0) / F::new(9.0) * t2540 * t5100 - F::new(16.0) * t87 * t82 + t19653);
    (t19642, t19645, t19655)
}
