//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1045/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1045<F: Float>(t143: F, t10742: F, t10674: F, t10677: F, t10682: F, t10685: F, t10690: F, t10693: F, t3182: F, t3205: F, t3238: F, t693: F, t707: F, t711: F, t715: F, t719: F, t723: F, t727: F, t731: F, t735: F) -> (F, F) {
    let t145 = 0.135e1 < t143;
    let t10743 = piecewise3(t145, t10742, 0.0);
    let t10760 = t10674 * t707 / 0.21233664e9 + t10677 * t707 / 412876800.0 - t3238 * t3205 / 0.37158912e10 - t10682 * t707 / 0.74317824e10 - 2.0 / 3.0 * t10685 * t707 + t3182 * t3205 / 3.0 + t10690 * t707 / 6.0 + t10693 * t707 / 8.0 - t693 * t10743 / 18.0 + t711 * t10743 / 240.0 - t715 * t10743 / 4480.0 + t719 * t10743 / 103680.0 - t723 * t10743 / 2838528.0 + t727 * t10743 / 89456640.0 - t731 * t10743 / 0.31850496e10 + t735 * t10743 / 0.1263403008e12;
    (t10743, t10760)
}
