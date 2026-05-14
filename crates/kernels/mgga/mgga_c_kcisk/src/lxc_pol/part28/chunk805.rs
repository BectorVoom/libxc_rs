//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 805/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk805<F: Float>(t9679: F, t9945: F, t1799: F, t2508: F, t2789: F, t415: F, t2528: F, t9687: F, t2454: F, t716: F, t719: F, t705: F, t2537: F, t717: F, t2785: F, t9649: F, t9662: F, t9664: F, t9678: F, t9918: F, t9922: F, t9927: F, t9932: F, t9936: F, t9940: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9946 = t9679 * t9945;
    let t9947 = t1799 * t9946;
    let t9949 = t2508 * t2789;
    let t9950 = t415 * t9949;
    let t9952 = t9687 * t2528;
    let t9953 = t415 * t9952;
    let t9956 = t716 * t2454 * t719;
    let t9957 = t705 * t9956;
    let t9958 = t415 * t9957;
    let t9960 = t717 * t2537;
    let t9961 = t415 * t9960;
    let t9963 = -0.10416666666666666667e-1 * t9918 * t2785 + 0.40208333333333333335e-2 * t9649 * t9922 - 0.10416666666666666667e-1 * t9927 * t2785 + 0.27777777777777777779e-1 * t9932 * t2785 - t9662 - 0.34722222222222222223e-2 * t9664 * t9936 + 0.10416666666666666667e-1 * t9664 * t9940 + 0.10416666666666666667e-1 * t9664 * t9922 + t9678 + 0.16581944444444444444e-2 * t9947 + 0.24872916666666666666e-2 * t9950 - 0.24872916666666666666e-2 * t9953 - 0.66327777777777777776e-2 * t9958 + 0.16581944444444444444e-2 * t9961;
    (t9946, t9947, t9949, t9950, t9952, t9953, t9956, t9957, t9958, t9960, t9961, t9963)
}
