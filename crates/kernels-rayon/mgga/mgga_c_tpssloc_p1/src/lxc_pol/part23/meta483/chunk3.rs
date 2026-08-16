//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1466/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1466(t11668: f64, t11678: f64, t11692: f64, t15569: f64, t15740: f64, t1653: f64, t19080: f64, t22158: f64, t22312: f64, t3578: f64, t45114: f64, t52680: f64, t5971: f64, t5975: f64, t6221: f64, t6225: f64, t6230: f64, t65819: f64, t72512: f64, t72530: f64, t72542: f64, t72556: f64, t72560: f64) -> f64 {
    let t79087 = 5.0_f64 / 1152.0_f64 * t15740 * t22158 + 5.0_f64 / 1728.0_f64 * t72512 + t45114 * t3578 * t22312 * t1653 / 192.0_f64 - t72530 / 288.0_f64 - t52680 / 3888.0_f64 - t11678 * t3578 * t6225 * t5975 / 192.0_f64 + t72542 / 54.0_f64 + 5.0_f64 / 1152.0_f64 * t11678 * t11668 * t6225 * t5971 + t65819 / 1728.0_f64 - 5.0_f64 / 216.0_f64 * t15569 * t22158 + t11692 * t3578 * t6230 * t5975 / 384.0_f64 - t19080 * t6221 / 48.0_f64 - t72556 / 576.0_f64 + 5.0_f64 / 864.0_f64 * t72560;
    t79087
}
