//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3177/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3177(t15643: f64, t5024: f64, t19201: f64, t3576: f64, t3577: f64, t44951: f64, t6191: f64, t11668: f64, t15569: f64, t15663: f64, t15704: f64, t15708: f64, t15750: f64, t18210: f64, t18231: f64, t19056: f64, t3494: f64, t3515: f64, t3580: f64, t44847: f64, t4582: f64, t52666: f64, t52674: f64, t52680: f64, t52682: f64, t52684: f64, t52766: f64, t52879: f64) -> f64 {
    let t65803 = t5024 * t15643;
    let t65815 = t19201 * t3576;
    let t65819 = t3577 * t44951 * t6191;
    let t65835 = t65803 / 162.0_f64 + t52666 / 324.0_f64 - t44847 / 972.0_f64 - t52674 / 216.0_f64 - t52680 / 7776.0_f64 + t52682 / 1152.0_f64 - t3515 * t4582 * t19056 * t3494 / 3072.0_f64 - t52684 / 864.0_f64 - t65815 * t3580 / 2304.0_f64 + t65819 / 10368.0_f64 - t52879 * t15663 / 576.0_f64 + t52766 * t15704 / 1152.0_f64 - 5.0_f64 / 648.0_f64 * t15569 * t15750 + 5.0_f64 / 6912.0_f64 * t3577 * t11668 * t18231 * t15708 + 5.0_f64 / 1152.0_f64 * t3577 * t11668 * t18210 * t15708;
    t65835
}
