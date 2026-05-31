//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1132/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1132<F: Float>(t2490: F, t62: F, t9047: F, t91794: F, t91796: F, t91799: F, t91801: F, t91804: F, t91806: F, t91809: F, t91811: F, t91814: F, t91816: F, t91818: F, t91820: F, t91822: F) -> (F, F) {
    let t91825 = t2490 * t62 * t9047;
    let t91827 = -F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t91794 - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t91796 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t91799 + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t91801 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t91804 - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t91806 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t91809 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t91811 + t91814 / F::cast_from(32.0_f64) + F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t91816 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t91818 - F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t91820 - F::cast_from(9.0_f64) / F::cast_from(4.0_f64) * t91822 - F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t91825;
    (t91825, t91827)
}
