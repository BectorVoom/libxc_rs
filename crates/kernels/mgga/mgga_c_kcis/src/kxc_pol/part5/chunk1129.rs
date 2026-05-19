//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1129/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1129<F: Float>(t13710: F, t13712: F, t13717: F, t13842: F, t18645: F, t18650: F, t18655: F, t18659: F, t18661: F, t18664: F, t18667: F, t18669: F, t18674: F, t18679: F, t18683: F, t9691: F, t9790: F) -> F {
    let t19040 = -t9790 - F::cast_from(0.79148148148148148147e-2_f64) * t9691 - F::cast_from(0.15829629629629629629e-1_f64) * t13710 + F::cast_from(0.79148148148148148147e-2_f64) * t13712 - t13842 + F::cast_from(0.23744444444444444444e-1_f64) * t13717 + F::cast_from(0.39574074074074074073e-2_f64) * t18645 - F::cast_from(0.19787037037037037037e-1_f64) * t18650 + F::cast_from(0.71233333333333333332e-1_f64) * t18655 - F::cast_from(0.47488888888888888888e-1_f64) * t18659 - F::cast_from(0.11872222222222222222e-1_f64) * t18661 - F::new(0.10685e0) * t18664 + F::cast_from(0.14246666666666666666e0_f64) * t18667 + F::cast_from(0.5936111111111111111e-2_f64) * t18669 - F::cast_from(0.11872222222222222222e-1_f64) * t18674 + F::cast_from(0.35616666666666666666e-1_f64) * t18679 - F::cast_from(0.17808333333333333333e-1_f64) * t18683;
    t19040
}
