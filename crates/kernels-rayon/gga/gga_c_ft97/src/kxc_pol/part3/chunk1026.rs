//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1026/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1026(t19862: f64, t871: f64, t296: f64, t4129: f64, t992: f64, t2875: f64, t2874: f64, t1212: f64, t18: f64, t10447: f64, t5414: f64, t10749: f64, t10773: f64, t11593: f64, t15467: f64, t15471: f64, t15491: f64, t15500: f64, t15502: f64, t15532: f64, t1901: f64, t19811: f64, t19816: f64, t19819: f64, t446: f64) -> (f64, f64) {
    let t19863 = t871 * t19862;
    let t19864 = t296 * t19863;
    let t19867 = t992 * t4129;
    let t19868 = t2875 * t19867;
    let t19869 = t2874 * t19868;
    let t19872 = t18 * t1212;
    let t19873 = t2875 * t19872;
    let t19874 = t2874 * t19873;
    let t19877 = t10447 * t5414;
    let t19880 = -t10749 + 2.0_f64 / 3.0_f64 * t446 * t19811 - t15467 + t15471 + t15491 - 4.0_f64 / 27.0_f64 * t10773 - t15500 - t15502 - 2.0_f64 / 9.0_f64 * t1901 * t19816 - 4.0_f64 / 9.0_f64 * t1901 * t19819 - t15532 - t446 * t19864 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t19869 - 4.0_f64 / 9.0_f64 * t11593 * t19874 + 2.0_f64 / 9.0_f64 * t1901 * t19877;
    (t19863, t19880)
}
