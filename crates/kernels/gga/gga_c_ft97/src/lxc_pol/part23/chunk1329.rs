//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1329/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1329<F: Float>(t19460: F, t25140: F, t18514: F, t25037: F, t18497: F, t31795: F, t8392: F, t10443: F, t10703: F, t112680: F, t113856: F, t113866: F, t113867: F, t11593: F, t15229: F, t15299: F, t15312: F, t15369: F, t18997: F, t1901: F, t19409: F, t19465: F, t19630: F, t24908: F, t2874: F, t31720: F, t31835: F, t446: F, t4969: F, t5424: F, t6260: F, t6360: F, t684: F, t840: F, t99238: F) -> (F, F, F, F) {
    let t126389 = t25140 * t19460;
    let t126397 = t25037 * t18514;
    let t126401 = t25140 * t18497;
    let t126405 = t8392 * t31795;
    let t126417 = -2.0 / 9.0 * t1901 * t10443 * t31720 - 2.0 / 9.0 * t1901 * t2874 * t24908 * t4969 - t113856 + t113866 - 8.0 / 27.0 * t113867 - 4.0 / 3.0 * t1901 * t15369 * t6360 * t19409 - 4.0 / 9.0 * t1901 * t15312 * t31835 * t684 + 4.0 / 9.0 * t1901 * t15299 * t126389 + 2.0 / 9.0 * t1901 * t10703 * t25140 * t19465 + 2.0 / 3.0 * t1901 * t15229 * t126397 + 8.0 / 9.0 * t11593 * t15229 * t126401 + 4.0 / 27.0 * t126405 - 4.0 / 9.0 * t1901 * t112680 * t18997 - 2.0 / 9.0 * t1901 * t99238 * t19630 - t446 * t840 * t5424 * t6260 / 3.0;
    (t126389, t126397, t126401, t126417)
}
