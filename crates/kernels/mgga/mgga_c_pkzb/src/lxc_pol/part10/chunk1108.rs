//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1108/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1108<F: Float>(t890: F, t9929: F, t6090: F, t6249: F, t7955: F, t8045: F, t9772: F, t9774: F, t9777: F, t9782: F, t9797: F, t9799: F, t9806: F, t9808: F, t6177: F, t6256: F, t7950: F, t8059: F, t8060: F, t9812: F, t9814: F, t9819: F, t9823: F, t9826: F, t9830: F, t9834: F) -> (F, F, F) {
    let t9930 = t9929 * t890;
    let t9947 = 0.264729375e1 * t9772 - 0.3529725e1 * t9774 - 0.17648625e1 * t9777 + 0.3529725e1 * t9799 - t6249 + 0.68863333333333333333e0 * t6090 + 0.13772666666666666667e1 * t7955 - t8045 - 0.516475e0 * t9782 + 0.1549425e1 * t9797 - 0.157790625e0 * t9806 + 0.6311625e0 * t9808;
    let t9957 = 0.31558125e0 * t9812 + 0.6311625e0 * t9814 - t6256 + 0.34731666666666666666e0 * t6177 + 0.69463333333333333333e0 * t7950 - t8059 - t8060 - 0.20839e0 * t9819 + 0.62517e0 * t9823 - 0.20839e0 * t9826 + 0.312585e0 * t9830 + 0.312585e0 * t9834;
    (t9930, t9947, t9957)
}
