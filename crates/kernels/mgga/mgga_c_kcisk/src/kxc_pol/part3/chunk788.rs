//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 788/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk788<F: Float>(t1212: F, t12885: F, t3722: F, t12974: F, t12922: F, t12927: F, t12929: F, t12931: F, t12933: F, t12948: F, t12954: F, t12959: F, t12985: F, t12989: F, t321: F, t1201: F, t13050: F, t13053: F, t13056: F, t13060: F, t13066: F, t13101: F, t3692: F, t3699: F, t3718: F, t3726: F) -> (F, F, F) {
    let t13105 = t3722 * t12885 * t1212;
    let t13110 = 0.55403703703703703703e-1 * t12974;
    let t13121 = -t13110 - 0.23744444444444444444e-1 * t12929 + 0.11872222222222222222e-1 * t12933 - 0.35616666666666666666e-1 * t12948 + 0.17808333333333333333e-1 * t12931 - 0.19787037037037037037e-1 * t12922 + 0.71233333333333333332e-1 * t12954 - 0.35616666666666666666e-1 * t12985 - 0.10685e0 * t12959 + 0.10685e0 * t12989 - 0.17808333333333333333e-1 * t12927;
    let t13123 = 0.62182e-1 * t13121 * t321;
    let t13124 = -0.17544670192365612213e1 * t3692 * t3718 - t13050 + t13053 - t13056 + t13060 - 0.51947267698127589899e2 * t3692 * t3726 + 0.1038945353962551798e3 * t1201 * t13066 - 0.58482233974552040708e0 * t1201 * t13101 - 0.35089340384731224426e1 * t1201 * t13105 + 0.35089340384731224426e1 * t3692 * t3699 - t13123;
    (t13105, t13123, t13124)
}
