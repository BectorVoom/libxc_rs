//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1094/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1094<F: Float>(t13712: F, t10218: F, t13710: F, t13714: F, t13723: F, t13732: F, t13767: F, t13772: F, t13775: F, t13777: F, t9700: F, t13717: F, t13720: F, t13726: F, t13729: F, t13735: F, t13738: F, t13742: F, t9681: F, t9683: F, t9691: F) -> F {
    let t13781 = F::cast_from(0.18344444444444444444e-2_f64) * t13712;
    let t13782 = -F::cast_from(0.27516666666666666666e-2_f64) * t9700 + F::cast_from(0.1982e-1_f64) * t13767 + F::cast_from(0.1651e-1_f64) * t13723 - F::cast_from(0.24765e-1_f64) * t13732 + F::cast_from(0.14865e-1_f64) * t13772 - t10218 - F::cast_from(0.1982e-1_f64) * t13775 - F::cast_from(0.991e-2_f64) * t13777 - F::cast_from(0.18344444444444444444e-2_f64) * t13710 - F::cast_from(0.55033333333333333333e-2_f64) * t13714 + t13781;
    let t13783 = -F::cast_from(0.27516666666666666667e-2_f64) * t13729 - F::cast_from(0.45861111111111111112e-2_f64) * t13720 - F::cast_from(0.11006666666666666667e-1_f64) * t13726 + F::cast_from(0.8255e-2_f64) * t13738 + F::cast_from(0.3302e-1_f64) * t13735 + F::cast_from(0.30268333333333333334e-1_f64) * t13717 - F::cast_from(0.8255e-2_f64) * t13742 + F::cast_from(0.13758333333333333333e-2_f64) * t9681 + F::cast_from(0.9172222222222222222e-3_f64) * t9683 - F::cast_from(0.36688888888888888888e-2_f64) * t9691 + t13782;
    t13783
}
