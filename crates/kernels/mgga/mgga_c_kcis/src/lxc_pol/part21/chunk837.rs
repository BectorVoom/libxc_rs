//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 837/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk837<F: Float>(t13806: F, t4764: F, t10974: F, t4763: F, t1692: F, t9630: F, t3006: F, t9634: F, t3005: F, t4758: F, t1226: F, t9825: F, t3593: F, t13714: F, t13710: F, t13712: F, t13717: F, t13720: F, t13723: F, t13726: F, t13729: F, t13732: F, t13735: F, t13738: F, t13742: F, t9681: F, t9683: F, t9691: F, t9700: F, t9790: F) -> (F, F, F, F, F, F) {
    let t13807 = t13806 * t4764;
    let t13812 = t4763 * t10974;
    let t13817 = t9630 * t1692;
    let t13818 = t9634 * t3006;
    let t13819 = t13817 * t13818;
    let t13822 = t3005 * t4758;
    let t13823 = t13822 * t1226;
    let t13826 = t9825 * t1692;
    let t13827 = t13826 * t3593;
    let t13842 = 0.23744444444444444444e-1 * t13714;
    let t13852 = -t9790 - 0.15829629629629629629e-1 * t9691 + 0.39574074074074074073e-2 * t9683 - 0.11872222222222222222e-1 * t9700 + 0.5936111111111111111e-2 * t9681 - 0.79148148148148148146e-2 * t13710 + 0.79148148148148148146e-2 * t13712 - t13842 + 0.13059444444444444444e0 * t13717 - 0.19787037037037037037e-1 * t13720 + 0.71233333333333333332e-1 * t13723 - 0.47488888888888888888e-1 * t13726 - 0.11872222222222222222e-1 * t13729 - 0.10685e0 * t13732 + 0.14246666666666666666e0 * t13735 + 0.35616666666666666666e-1 * t13738 - 0.35616666666666666666e-1 * t13742;
    (t13807, t13812, t13819, t13823, t13827, t13852)
}
