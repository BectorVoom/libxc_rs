//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 986/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk986<F: Float>(t25: F, t6184: F, t1599: F, t4429: F, t6141: F, t18119: F, t5426: F, t12617: F, t5440: F, t4440: F, t12825: F, t2099: F) -> (F, F, F, F, F, F, F) {
    let t18146 = t25 * t6184;
    let t18148 = t1599 * t18146 / F::cast_from(288.0_f64);
    let t18152 = t6141 * t4429 / F::cast_from(108.0_f64);
    let t18155 = t5426 * t18119;
    let t18156 = t12617 * t18155;
    let t18159 = t5440 * t18119;
    let t18160 = t4440 * t18159;
    let t18163 = t12825 * t2099;
    (t18148, t18152, t18155, t18156, t18159, t18160, t18163)
}
