//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1237/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1237<F: Float>(t5415: F, t5412: F, t5400: F, t6262: F, t4544: F, t4528: F, t13031: F, t2653: F, t2796: F, t2800: F, t2805: F, t3706: F, t3711: F, t3714: F, t4507: F, t8521: F) -> (F,) {
    let t18422 = t5415 / 8.0;
    let t18423 = t5412 / 8.0;
    let t18424 = t5400 / 8.0;
    let t18425 = t6262 / 8.0;
    let t18426 = t4544 / 8.0;
    let t18427 = 2.0 * t4528;
    let t18428 = 4.0 * t2653 + t13031 - t18422 + t2800 - t18423 - t2796 - t18424 + t8521 - t4507 - t3714 - t3711 - t3706 - t18425 - t18426 + t18427 - t2805;
    (t18428,)
}
