//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 763/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk763(t1094: f64, t12517: f64, t12534: f64, t12564: f64, t12568: f64, t12573: f64, t12581: f64, t12584: f64, t12589: f64, t12592: f64, t12595: f64, t15559: f64, t15610: f64, t240: f64, t3357: f64) -> f64 {
    let t15613 = 0.1038945353962551798e3_f64 * t1094 * t12568 - t12517 + t12534 + t12564 - 0.21687161765563048428e-1_f64 * t3357 * t12589 + 0.16265371324172286321e-1_f64 * t3357 * t12592 + 0.48159446095139119799e0_f64 * t3357 * t12595 - 0.51947267698127589897e2_f64 * t1094 * t12573 + t240 * (t15559 + t15610) - t12581 + t12584;
    t15613
}
