//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 519/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk519<F: Float>(t1673: F, t3021: F, t1043: F, t677: F, t2970: F, t2976: F, t2984: F, t2988: F, t2991: F, t3002: F, t3009: F, t3014: F, t3019: F) -> (F, F) {
    let t3022 = t3021 * t1673;
    let t3023 = t1043 * t3022;
    let t3025 = t1043 * t677;
    let t3027 = F::cast_from(0.54106179813099907243e-4_f64) * t2970 + F::cast_from(0.6081664768516204948e-3_f64) * t2976 - F::cast_from(0.10120768229166666667e-4_f64) * t2984 + F::cast_from(0.86880925264517213544e-4_f64) * t2988 + F::cast_from(0.86880925264517213544e-4_f64) * t2991 + F::cast_from(0.84412963981222021454e-7_f64) * t3002 + F::cast_from(0.16882592796244404291e-6_f64) * t3009 - F::cast_from(0.25340269868817520617e-4_f64) * t3014 - F::cast_from(0.72463633678258804342e-6_f64) * t3019 + F::cast_from(0.14492726735651760868e-5_f64) * t3023 - F::cast_from(0.84540905957968605066e-5_f64) * t3025;
    (t3022, t3027)
}
