//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 706/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk706(t142: f64, t2855: f64, t2858: f64, t298: f64, t56: f64, t69: f64, t918: f64, t12588: f64, t12589: f64, t12592: f64, t12595: f64, t12601: f64, t12604: f64, t12608: f64, t12614: f64, t12620: f64, t2895: f64, t834: f64, t839: f64) -> (f64, f64, f64) {
    let t12624 = 0.10685e0_f64 * t298 * t142 * t2855 * t2858;
    let t12626 = t69 * t918 * t56;
    let t12629 = t12588 - 0.21687161765563048428e-1_f64 * t2895 * t12589 + 0.16265371324172286321e-1_f64 * t2895 * t12592 + 0.48159446095139119799e0_f64 * t2895 * t12595 + t12601 - t12604 - t12608 - 0.1025389702100779493e4_f64 * t839 * t12614 + t12620 + t12624 - 0.56969282336565386482e-3_f64 * t834 * t12626;
    (t12624, t12626, t12629)
}
