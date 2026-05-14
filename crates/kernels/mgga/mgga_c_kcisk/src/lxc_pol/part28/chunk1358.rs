//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1358/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1358<F: Float>(t112221: F, t1790: F, t7261: F, t8851: F, t1785: F, t32935: F, t8845: F, t33003: F, t10494: F, t35132: F, t112591: F, t116351: F, t121031: F, t121219: F, t121222: F, t121226: F, t121229: F, t121236: F, t121241: F, t32942: F, t32990: F, t33002: F, t35212: F, t9649: F, t9667: F, t9672: F) -> (F, F, F, F, F) {
    let t121246 = t7261 * t112221 * t8851 * t1790;
    let t121253 = t7261 * t32935 * t8845 * t1785;
    let t121258 = t7261 * t33003 * t8845 * t1790;
    let t121265 = t10494 * t35132;
    let t121267 = -0.66327777777777777776e-2 * t121219 + 0.16581944444444444444e-2 * t121222 + 0.16581944444444444444e-2 * t121226 + 0.33163888888888888888e-2 * t121229 - 0.55555555555555555558e-1 * t121031 * t9672 - 0.46296296296296296297e-2 * t116351 + 0.23280625000000000001e-2 * t33002 * t121236 - 0.46561250000000000002e-2 * t33002 * t121241 + 0.17972642500000000001e-2 * t112591 * t121246 + 0.18518518518518518519e-1 * t121031 * t9667 - 0.40208333333333333335e-2 * t9649 * t121253 - 0.120625e-1 * t9649 * t121258 + 0.10416666666666666667e-1 * t32942 * t35212 + 0.10416666666666666667e-1 * t32990 * t35212 + 0.18424382716049382715e-2 * t121265;
    (t121246, t121253, t121258, t121265, t121267)
}
