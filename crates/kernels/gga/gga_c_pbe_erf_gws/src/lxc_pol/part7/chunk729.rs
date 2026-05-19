//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 729/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk729<F: Float>(t164: F, t5984: F, t1964: F, t528: F, t163: F, t169: F, t171: F, t4563: F, t5891: F, t5895: F, t5898: F, t5962: F, t5969: F, t5973: F, t5977: F, t5980: F, t5982: F) -> (F, F) {
    let t5985 = t5984 * t164;
    let t5986 = F::cast_from(0.1186530987165140469e-3_f64) * t5985;
    let t5988 = F::cast_from(0.94516221669423353502e-1_f64) * t528 * t1964;
    let t5989 = -F::cast_from(0.14862827083471493416e-2_f64) * t5891 - t5895 - t5898 - F::cast_from(0.53884053046145740922e-2_f64) * t169 * t171 * t5962 * t163 - F::cast_from(0.71845404061527654564e-1_f64) * t5969 + F::cast_from(0.26942026523072870461e-1_f64) * t5973 - t5977 - F::cast_from(0.31505407223141117834e-1_f64) * t4563 * t164 - F::cast_from(0.94516221669423353502e-1_f64) * t5980 - F::cast_from(0.94516221669423353502e-1_f64) * t5982 - t5986 + t5988;
    (t5985, t5989)
}
