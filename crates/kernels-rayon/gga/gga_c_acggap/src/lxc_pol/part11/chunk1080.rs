//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1080/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1080(t17912: f64, t2302: f64, t31443: f64, t3176: f64, t1530: f64, t31056: f64, t13287: f64, t33953: f64, t5136: f64, t5141: f64, t15386: f64, t30730: f64, t30738: f64, t30744: f64, t30748: f64, t30750: f64, t30756: f64, t30758: f64, t30763: f64, t30767: f64, t34795: f64, t34798: f64, t34803: f64, t34804: f64, t34817: f64) -> (f64, f64, f64) {
    let t34821 = t31443 * t17912 * t2302 * t3176;
    let t34823 = t1530 * t31056;
    let t34826 = t34823 * t13287 * t33953 * t5136;
    let t34828 = t33953 * t5141;
    let t34830 = t34823 * t15386 * t34828;
    let t34832 = t34795 + 0.7862023072401038017e-3_f64 * t34798 + t34803 + 0.10482697429868050689e-2_f64 * t34804 - 0.62896184579208304136e-3_f64 * t30730 - 0.31448092289604152068e-2_f64 * t30738 - 0.94344276868812456204e-3_f64 * t30744 + 0.41930789719472202756e-3_f64 * t30748 - 0.17149607247227894789e-2_f64 * t30750 + 0.17149607247227894789e-2_f64 * t30756 + 0.62896184579208304136e-3_f64 * t30758 + 7.0_f64 / 72.0_f64 * t30763 + 7.0_f64 / 144.0_f64 * t30767 - 0.31448092289604152068e-2_f64 * t34817 - 0.18868855373762491241e-2_f64 * t34821 - 0.12579236915841660827e-2_f64 * t34826 + 0.18868855373762491241e-2_f64 * t34830;
    (t34823, t34828, t34832)
}
