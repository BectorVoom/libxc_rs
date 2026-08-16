//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3500/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3500<F: Float>(t19697: F, t3173: F, t1042: F, t1063: F, t11703: F, t13392: F, t15725: F, t15758: F, t15935: F, t16095: F, t16096: F, t18903: F, t19651: F, t19663: F, t19688: F, t19800: F, t19973: F, t20099: F, t2853: F, t3059: F, t3106: F, t3127: F, t3169: F, t3181: F, t42410: F, t4837: F, t4872: F, t51963: F, t53661: F, t5825: F, t6258: F, t65370: F, t65947: F) -> F {
    let t66003 = t19697 * t3173;
    let t66013 = F::cast_from(0.28582678745379824648e-3_f64) * t4837 * t1042 * t4872 * t5825 * t3059 + F::cast_from(0.17149607247227894789e-2_f64) * t15758 * t19973 - F::cast_from(0.1270341277572436651e-2_f64) * t16095 * t42410 * t18903 * t16096 + F::cast_from(0.11433071498151929859e-2_f64) * t53661 - F::cast_from(0.47637797908966374413e-3_f64) * t16095 * t11703 * t20099 * t13392 + F::cast_from(0.85748036236139473944e-3_f64) * t1063 * t1042 * t15935 * t65370 - F::cast_from(0.2540682555144873302e-2_f64) * t3106 * t19688 - F::cast_from(0.23818898954483187207e-3_f64) * t3127 * t1042 * t3181 * t6258 * t2853 + F::cast_from(0.57165357490759649296e-3_f64) * t15725 * t19651 + F::cast_from(0.28582678745379824648e-3_f64) * t66003 + F::cast_from(0.15244095330869239812e-1_f64) * t3106 * t19663 - F::cast_from(0.22866142996303859718e-2_f64) * t3169 * t19800 + F::cast_from(0.85748036236139473944e-2_f64) * t1063 * t1042 * t51963 * t65947;
    t66013
}
