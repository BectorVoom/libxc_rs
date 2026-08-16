//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1098/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1098(t13817: f64, t13818: f64, t3005: f64, t4758: f64, t1226: f64, t1692: f64, t9825: f64, t3593: f64, t13714: f64, t13710: f64, t13712: f64, t13717: f64, t13720: f64, t13723: f64, t13726: f64, t13729: f64, t13732: f64, t13735: f64, t13738: f64, t13742: f64, t9681: f64, t9683: f64, t9691: f64, t9700: f64, t9790: f64) -> (f64, f64, f64, f64) {
    let t13819 = t13817 * t13818;
    let t13822 = t3005 * t4758;
    let t13823 = t13822 * t1226;
    let t13826 = t9825 * t1692;
    let t13827 = t13826 * t3593;
    let t13842 = 0.23744444444444444444e-1_f64 * t13714;
    let t13852 = -t9790 - 0.15829629629629629629e-1_f64 * t9691 + 0.39574074074074074073e-2_f64 * t9683 - 0.11872222222222222222e-1_f64 * t9700 + 0.5936111111111111111e-2_f64 * t9681 - 0.79148148148148148146e-2_f64 * t13710 + 0.79148148148148148146e-2_f64 * t13712 - t13842 + 0.13059444444444444444e0_f64 * t13717 - 0.19787037037037037037e-1_f64 * t13720 + 0.71233333333333333332e-1_f64 * t13723 - 0.47488888888888888888e-1_f64 * t13726 - 0.11872222222222222222e-1_f64 * t13729 - 0.10685e0_f64 * t13732 + 0.14246666666666666666e0_f64 * t13735 + 0.35616666666666666666e-1_f64 * t13738 - 0.35616666666666666666e-1_f64 * t13742;
    (t13819, t13823, t13827, t13852)
}
