//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1098/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1098<F: Float>(t13817: F, t13818: F, t3005: F, t4758: F, t1226: F, t1692: F, t9825: F, t3593: F, t13714: F, t13710: F, t13712: F, t13717: F, t13720: F, t13723: F, t13726: F, t13729: F, t13732: F, t13735: F, t13738: F, t13742: F, t9681: F, t9683: F, t9691: F, t9700: F, t9790: F) -> (F, F, F, F) {
    let t13819 = t13817 * t13818;
    let t13822 = t3005 * t4758;
    let t13823 = t13822 * t1226;
    let t13826 = t9825 * t1692;
    let t13827 = t13826 * t3593;
    let t13842 = F::cast_from(0.23744444444444444444e-1_f64) * t13714;
    let t13852 = -t9790 - F::cast_from(0.15829629629629629629e-1_f64) * t9691 + F::cast_from(0.39574074074074074073e-2_f64) * t9683 - F::cast_from(0.11872222222222222222e-1_f64) * t9700 + F::cast_from(0.5936111111111111111e-2_f64) * t9681 - F::cast_from(0.79148148148148148146e-2_f64) * t13710 + F::cast_from(0.79148148148148148146e-2_f64) * t13712 - t13842 + F::cast_from(0.13059444444444444444e0_f64) * t13717 - F::cast_from(0.19787037037037037037e-1_f64) * t13720 + F::cast_from(0.71233333333333333332e-1_f64) * t13723 - F::cast_from(0.47488888888888888888e-1_f64) * t13726 - F::cast_from(0.11872222222222222222e-1_f64) * t13729 - F::cast_from(0.10685e0_f64) * t13732 + F::cast_from(0.14246666666666666666e0_f64) * t13735 + F::cast_from(0.35616666666666666666e-1_f64) * t13738 - F::cast_from(0.35616666666666666666e-1_f64) * t13742;
    (t13819, t13823, t13827, t13852)
}
