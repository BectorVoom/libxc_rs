//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1442/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1442(t15776: f64, t5096: f64, t1162: f64, t12635: f64, t15850: f64, t15856: f64, t15889: f64, t17724: f64, t17982: f64, t18055: f64, t18062: f64, t18066: f64, t18072: f64, t3087: f64, t3234: f64, t3235: f64, t35733: f64, t35745: f64, t4444: f64, t5298: f64, t5302: f64, t55613: f64, t55623: f64, t55625: f64, t55637: f64, t58322: f64, t58865: f64, t8482: f64, t914: f64) -> (f64, f64) {
    let t60141 = t15776 * t5096;
    let t60168 = 0.15454509315180013964e0_f64 * t55613 + 0.15454509315180013964e0_f64 * t1162 * t914 * t3087 * t58865 + 0.2339219295794108718e2_f64 * t3234 * t3235 * t60141 + 0.54090782603130048873e0_f64 * t1162 * t914 * t8482 * t58322 - 0.16156588482142000549e2_f64 * t12635 * t18072 - 0.23181763972770020945e0_f64 * t55623 - 0.33268896651293990656e3_f64 * t55625 - 0.25565825668348355228e6_f64 * t35745 * t17982 + 0.17017482394825239973e1_f64 * t15889 * t5302 + 0.36282051390366161644e7_f64 * t35733 * t18062 - 0.28131159491972598278e5_f64 * t15850 * t18066 + 0.14065579745986299139e5_f64 * t15856 * t18055 - 0.47768371634597164836e0_f64 * t4444 * t17724 + 0.10210489436895143984e1_f64 * t15889 * t5298 - 0.12117441361606500412e2_f64 * t55637;
    (t60141, t60168)
}
