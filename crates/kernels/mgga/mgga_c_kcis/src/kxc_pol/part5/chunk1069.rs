//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1069/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1069<F: Float>(t18091: F, t4439: F, t1607: F, t5713: F, t110: F, t2105: F, t1599: F, t25: F, t6184: F, t4429: F, t6141: F, t12825: F, t2099: F) -> (F, F, F, F, F, F) {
    let t18093 = t4439 * t18091 / F::new(864.0);
    let t18128 = t5713 * t1607;
    let t18141 = t110 * t2105;
    let t18142 = t1599 * t18141;
    let t18146 = t25 * t6184;
    let t18148 = t1599 * t18146 / F::new(288.0);
    let t18152 = t6141 * t4429 / F::new(108.0);
    let t18163 = t12825 * t2099;
    (t18093, t18128, t18142, t18148, t18152, t18163)
}
