//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1853/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1853<F: Float>(t19021: F, t964: F, t973: F, t981: F, t3022: F, t6227: F, t11528: F, t6110: F, t2869: F, t6142: F, t11134: F, t11560: F, t15189: F, t15483: F, t15484: F, t15485: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18944: F, t18948: F) -> (F, F, F, F, F, F) {
    let t19023 = t964 * t19021 * t973;
    let t19025 = F::cast_from(0.5848223622634646207e0_f64) * t981 * t19023;
    let t19027 = F::cast_from(0.17315859105681463759e2_f64) * t3022 * t6227;
    let t19029 = F::cast_from(2.0_f64) * t11528 * t6110;
    let t19031 = F::cast_from(1.0_f64) * t2869 * t6142;
    let t19045 = -t11560 - F::cast_from(0.41203703703703703703e-2_f64) * t11134 - F::cast_from(0.82407407407407407408e-2_f64) * t15189 + t15483 - t15484 + t15485 + F::cast_from(0.20601851851851851852e-2_f64) * t18919 - F::cast_from(0.10300925925925925926e-1_f64) * t18906 + F::cast_from(0.37083333333333333333e-1_f64) * t18911 - F::cast_from(0.12361111111111111111e-1_f64) * t18915 - F::cast_from(0.61805555555555555557e-2_f64) * t18924 - F::cast_from(0.55625000000000000001e-1_f64) * t18928 + F::cast_from(0.37083333333333333334e-1_f64) * t18932 + F::cast_from(0.30902777777777777778e-2_f64) * t18934 - F::cast_from(0.61805555555555555555e-2_f64) * t18939 + F::cast_from(0.18541666666666666667e-1_f64) * t18944 - F::cast_from(0.92708333333333333333e-2_f64) * t18948;
    (t19023, t19025, t19027, t19029, t19031, t19045)
}
