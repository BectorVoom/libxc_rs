//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1015/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1015<F: Float>(t30655: F, t334: F, t2093: F, t25663: F, t5715: F, t7786: F, t19476: F, t7789: F, t1201: F, t30553: F, t30557: F, t30561: F, t30564: F, t30567: F, t30641: F, t30644: F, t45: F) -> (F, F, F, F, F) {
    let t30656 = t30655 * t334;
    let t30660 = F::new(3.0) * t25663 * t2093;
    let t30662 = F::new(3.0) * t5715 * t7786;
    let t30664 = F::new(0.48245472966453314466e2) * t19476 * t7789;
    let t30665 = -F::new(0.35089340384731224426e1) * t1201 * t30553 - t30557 + t30561 - t30564 + t30567 + t30641 + t30644 + F::new(0.19751789702565206229e-1) * t45 * t30656 + t30660 + t30662 + t30664;
    (t30656, t30660, t30662, t30664, t30665)
}
