//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1080/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1080<F: Float>(t17912: F, t2302: F, t31443: F, t3176: F, t1530: F, t31056: F, t13287: F, t33953: F, t5136: F, t5141: F, t15386: F, t30730: F, t30738: F, t30744: F, t30748: F, t30750: F, t30756: F, t30758: F, t30763: F, t30767: F, t34795: F, t34798: F, t34803: F, t34804: F, t34817: F) -> (F, F, F) {
    let t34821 = t31443 * t17912 * t2302 * t3176;
    let t34823 = t1530 * t31056;
    let t34826 = t34823 * t13287 * t33953 * t5136;
    let t34828 = t33953 * t5141;
    let t34830 = t34823 * t15386 * t34828;
    let t34832 = t34795 + F::new(0.7862023072401038017e-3) * t34798 + t34803 + F::new(0.10482697429868050689e-2) * t34804 - F::new(0.62896184579208304136e-3) * t30730 - F::new(0.31448092289604152068e-2) * t30738 - F::new(0.94344276868812456204e-3) * t30744 + F::new(0.41930789719472202756e-3) * t30748 - F::new(0.17149607247227894789e-2) * t30750 + F::new(0.17149607247227894789e-2) * t30756 + F::new(0.62896184579208304136e-3) * t30758 + F::new(7.0) / F::new(72.0) * t30763 + F::new(7.0) / F::new(144.0) * t30767 - F::new(0.31448092289604152068e-2) * t34817 - F::new(0.18868855373762491241e-2) * t34821 - F::new(0.12579236915841660827e-2) * t34826 + F::new(0.18868855373762491241e-2) * t34830;
    (t34823, t34828, t34832)
}
