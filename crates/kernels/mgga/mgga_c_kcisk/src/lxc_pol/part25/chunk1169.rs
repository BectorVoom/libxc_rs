//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1169/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1169<F: Float>(t4830: F, t9926: F, t20: F, t654: F, t7201: F, t1693: F, t9660: F, t9927: F, t1863: F, t9956: F, t415: F, t2785: F, t34073: F, t34125: F, t34148: F, t34218: F, t34223: F, t34225: F, t9649: F, t9652: F, t9664: F, t9667: F, t9672: F) -> (F, F, F, F, F, F) {
    let t34228 = t4830 * t9926;
    let t34232 = t7201 * t654 * t20;
    let t34233 = t1693 * t34232;
    let t34236 = t9927 * t9660;
    let t34242 = t1863 * t9956;
    let t34243 = t415 * t34242;
    let t34251 = 0.40208333333333333335e-2 * t9649 * t34218 - 0.16581944444444444444e-2 * t34223 - 0.10722222222222222223e-1 * t34225 * t9652 - 0.10416666666666666667e-1 * t34228 * t2785 - 0.10416666666666666667e-1 * t34233 * t2785 - 0.34722222222222222223e-2 * t34236 - 0.27777777777777777779e-1 * t34125 * t9652 + 0.10416666666666666667e-1 * t9664 * t34218 - 0.66327777777777777776e-2 * t34243 - 0.27777777777777777779e-1 * t34125 * t9672 - 0.34722222222222222223e-2 * t34073 * t9667 - 0.40208333333333333335e-2 * t9649 * t34148;
    (t34228, t34232, t34233, t34242, t34243, t34251)
}
