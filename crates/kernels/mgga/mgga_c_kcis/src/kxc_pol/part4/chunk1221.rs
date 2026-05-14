//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1221/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1221<F: Float>(t25: F, t6184: F, t1599: F, t4429: F, t6141: F, t18119: F, t5426: F, t12617: F, t5440: F, t4440: F, t12825: F, t2099: F, t12844: F, t6155: F, t4439: F, t3970: F, t617: F) -> (F, F, F, F, F, F, F) {
    let t18146 = t25 * t6184;
    let t18148 = t1599 * t18146 / 288.0;
    let t18152 = t6141 * t4429 / 108.0;
    let t18155 = t5426 * t18119;
    let t18156 = t12617 * t18155;
    let t18159 = t5440 * t18119;
    let t18160 = t4440 * t18159;
    let t18163 = t12825 * t2099;
    let t18164 = t1599 * t18163;
    let t18168 = t12844 * t6155;
    let t18170 = t4439 * t18168 / 864.0;
    let t18171 = t3970 * t617;
    (t18148, t18152, t18156, t18160, t18164, t18170, t18171)
}
