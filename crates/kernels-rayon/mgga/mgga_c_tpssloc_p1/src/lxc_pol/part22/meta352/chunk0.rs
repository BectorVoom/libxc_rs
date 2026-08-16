//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1564/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1564(t5576: f64, t838: f64, t119: f64, t16662: f64, t210: f64, t4180: f64, t4181: f64, t4234: f64, t16839: f64, t829: f64, t16891: f64, t10014: f64, t10026: f64, t10029: f64, t10036: f64, t13359: f64, t13362: f64, t13368: f64, t16985: f64, t16988: f64, t16990: f64, t16993: f64, t16995: f64, t16997: f64, t249: f64, t2623: f64, t2643: f64, t5624: f64, t5628: f64, t787: f64, t843: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17000 = t5576 * t838;
    let t17003 = t119 * t16662;
    let t17004 = t210 * t17003;
    let t17009 = t4180 * t4181 * t4234;
    let t17013 = t4180 * t16839 * t829;
    let t17017 = t4180 * t16891 * t829;
    let t17020 = 5.0_f64 / 768.0_f64 * t2623 * t5624 - t2623 * t5628 / 768.0_f64 - t843 * t16985 / 768.0_f64 - 35.0_f64 / 1152.0_f64 * t16988 + 7.0_f64 / 576.0_f64 * t16990 + 119.0_f64 / 13824.0_f64 * t10014 - t10026 - 7.0_f64 / 48.0_f64 * t16993 + 7.0_f64 / 144.0_f64 * t16995 + t16997 * t249 / 3072.0_f64 - 7.0_f64 / 4608.0_f64 * t17000 - t10029 + t13359 + t13362 - 119.0_f64 / 1728.0_f64 * t13368 - t787 * t17004 / 48.0_f64 - 35.0_f64 / 216.0_f64 * t10036 - t2643 * t17009 / 1536.0_f64 - t2643 * t17013 / 3072.0_f64 - t2643 * t17017 / 3072.0_f64;
    (t17000, t17004, t17009, t17013, t17017, t17020)
}
