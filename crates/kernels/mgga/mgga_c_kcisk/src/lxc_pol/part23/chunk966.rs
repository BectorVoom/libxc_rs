//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 966/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk966<F: Float>(t12910: F, t19613: F, t3679: F, t5748: F, t1190: F, t3677: F, t3671: F, t5752: F, t2093: F, t3640: F, t19104: F, t12929: F, t12931: F, t12933: F, t12948: F, t13110: F, t19100: F, t19102: F, t19106: F, t19111: F, t19116: F, t19121: F, t19125: F, t19129: F, t19134: F, t19138: F, t19142: F) -> (F, F, F, F, F) {
    let t19615 = 0.96490945932906628932e2 * t12910 * t19613;
    let t19616 = t5748 * t3679;
    let t19617 = t19616 * t1190;
    let t19619 = 0.32163648644302209644e2 * t3677 * t19617;
    let t19620 = t5752 * t3671;
    let t19622 = 0.16081824322151104822e2 * t3677 * t19620;
    let t19623 = t2093 * t3640;
    let t19625 = 6.0 * t3677 * t19623;
    let t19632 = 0.23744444444444444444e-1 * t19104;
    let t19642 = -t13110 - 0.15829629629629629629e-1 * t12929 + 0.39574074074074074073e-2 * t12933 - 0.11872222222222222222e-1 * t12948 + 0.5936111111111111111e-2 * t12931 - 0.79148148148148148146e-2 * t19100 + 0.79148148148148148146e-2 * t19102 - t19632 + 0.13059444444444444444e0 * t19106 - 0.19787037037037037037e-1 * t19111 + 0.71233333333333333332e-1 * t19116 - 0.47488888888888888888e-1 * t19121 - 0.11872222222222222222e-1 * t19125 - 0.10685e0 * t19129 + 0.14246666666666666666e0 * t19134 + 0.35616666666666666666e-1 * t19138 - 0.35616666666666666666e-1 * t19142;
    (t19615, t19619, t19622, t19625, t19642)
}
