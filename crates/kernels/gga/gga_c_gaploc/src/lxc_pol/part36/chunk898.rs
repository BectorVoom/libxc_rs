//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 898/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk898<F: Float>(t105: F, t123: F, t12794: F, t12815: F, t1358: F, t169: F, t172: F, t380: F, t41801: F, t419: F, t42123: F, t42130: F, t42529: F, t42533: F, t42537: F, t42540: F, t42544: F, t42547: F, t42551: F, t42570: F, t42573: F, t42575: F, t452: F, t488: F, t492: F) -> F {
    let t42577 = -F::new(0.31616674039640166221e-2) * t1358 * t41801 * t123 * t488 + F::new(0.23712505529730124666e-2) * t42529 - F::new(0.63233348079280332442e-2) * t42533 + t42537 + t42540 + t42544 - t42547 - t42551 + F::new(0.28455006635676149599e-1) * t419 * t12815 + F::new(0.28455006635676149599e-1) * t105 * t452 * t42123 * t169 * t172 - F::new(0.28455006635676149599e-1) * t105 * t492 * t42130 - F::new(0.28455006635676149599e-1) * t419 * t12794 - F::new(0.37940008847568199465e-1) * t380 * t12794 + F::new(0.37940008847568199465e-1) * t380 * t12815 - t42570 - t42573 + F::new(0.18970004423784099733e-1) * t42575;
    t42577
}
