//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3500/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3500(t19697: f64, t3173: f64, t1042: f64, t1063: f64, t11703: f64, t13392: f64, t15725: f64, t15758: f64, t15935: f64, t16095: f64, t16096: f64, t18903: f64, t19651: f64, t19663: f64, t19688: f64, t19800: f64, t19973: f64, t20099: f64, t2853: f64, t3059: f64, t3106: f64, t3127: f64, t3169: f64, t3181: f64, t42410: f64, t4837: f64, t4872: f64, t51963: f64, t53661: f64, t5825: f64, t6258: f64, t65370: f64, t65947: f64) -> f64 {
    let t66003 = t19697 * t3173;
    let t66013 = 0.28582678745379824648e-3_f64 * t4837 * t1042 * t4872 * t5825 * t3059 + 0.17149607247227894789e-2_f64 * t15758 * t19973 - 0.1270341277572436651e-2_f64 * t16095 * t42410 * t18903 * t16096 + 0.11433071498151929859e-2_f64 * t53661 - 0.47637797908966374413e-3_f64 * t16095 * t11703 * t20099 * t13392 + 0.85748036236139473944e-3_f64 * t1063 * t1042 * t15935 * t65370 - 0.2540682555144873302e-2_f64 * t3106 * t19688 - 0.23818898954483187207e-3_f64 * t3127 * t1042 * t3181 * t6258 * t2853 + 0.57165357490759649296e-3_f64 * t15725 * t19651 + 0.28582678745379824648e-3_f64 * t66003 + 0.15244095330869239812e-1_f64 * t3106 * t19663 - 0.22866142996303859718e-2_f64 * t3169 * t19800 + 0.85748036236139473944e-2_f64 * t1063 * t1042 * t51963 * t65947;
    t66013
}
