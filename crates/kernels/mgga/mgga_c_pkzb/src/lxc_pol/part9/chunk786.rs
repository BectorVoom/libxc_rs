//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 786/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk786<F: Float>(t1843: F, t655: F, t218: F, t219: F, t208: F, t5537: F, t5513: F, t5516: F, t5522: F, t5525: F, t5539: F, t5541: F, t5543: F, t5548: F, t5551: F, t5553: F, t5558: F, t5560: F, t5563: F, t5566: F) -> (F, F, F, F, F) {
    let t5568 = t655 * t1843;
    let t5570 = t218 * t219 * t5568;
    let t5572 = t208 * t5537;
    let t5574 = t218 * t219 * t5572;
    let t5576 = F::new(0.19419375e1) * t5513 - F::new(0.3883875e1) * t5516 + F::new(0.258925e1) * t5541 - t5543 + F::new(0.12077e1) * t5522 - F::new(0.905775e0) * t5525 + F::new(0.905775e0) * t5539 - F::new(0.412621875e-1) * t5548 + F::new(0.247573125e0) * t5551 + F::new(0.16504875e0) * t5553 - t5558 + F::new(0.82785e0) * t5560 - F::new(0.49671e0) * t5563 - F::new(0.49671e0) * t5566 + F::new(0.745065e0) * t5570 + F::new(0.248355e0) * t5574;
    (t5568, t5570, t5572, t5574, t5576)
}
