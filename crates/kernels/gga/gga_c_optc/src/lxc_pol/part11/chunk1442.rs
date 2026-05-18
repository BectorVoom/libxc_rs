//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1442/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1442<F: Float>(t15776: F, t5096: F, t1162: F, t12635: F, t15850: F, t15856: F, t15889: F, t17724: F, t17982: F, t18055: F, t18062: F, t18066: F, t18072: F, t3087: F, t3234: F, t3235: F, t35733: F, t35745: F, t4444: F, t5298: F, t5302: F, t55613: F, t55623: F, t55625: F, t55637: F, t58322: F, t58865: F, t8482: F, t914: F) -> (F, F) {
    let t60141 = t15776 * t5096;
    let t60168 = F::new(0.15454509315180013964e0) * t55613 + F::new(0.15454509315180013964e0) * t1162 * t914 * t3087 * t58865 + F::new(0.2339219295794108718e2) * t3234 * t3235 * t60141 + F::new(0.54090782603130048873e0) * t1162 * t914 * t8482 * t58322 - F::new(0.16156588482142000549e2) * t12635 * t18072 - F::new(0.23181763972770020945e0) * t55623 - F::new(0.33268896651293990656e3) * t55625 - F::new(0.25565825668348355228e6) * t35745 * t17982 + F::new(0.17017482394825239973e1) * t15889 * t5302 + F::new(0.36282051390366161644e7) * t35733 * t18062 - F::new(0.28131159491972598278e5) * t15850 * t18066 + F::new(0.14065579745986299139e5) * t15856 * t18055 - F::new(0.47768371634597164836e0) * t4444 * t17724 + F::new(0.10210489436895143984e1) * t15889 * t5298 - F::new(0.12117441361606500412e2) * t55637;
    (t60141, t60168)
}
