//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 706/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk706<F: Float>(t142: F, t2855: F, t2858: F, t298: F, t56: F, t69: F, t918: F, t12588: F, t12589: F, t12592: F, t12595: F, t12601: F, t12604: F, t12608: F, t12614: F, t12620: F, t2895: F, t834: F, t839: F) -> (F, F, F) {
    let t12624 = F::new(0.10685e0) * t298 * t142 * t2855 * t2858;
    let t12626 = t69 * t918 * t56;
    let t12629 = t12588 - F::cast_from(0.21687161765563048428e-1_f64) * t2895 * t12589 + F::cast_from(0.16265371324172286321e-1_f64) * t2895 * t12592 + F::cast_from(0.48159446095139119799e0_f64) * t2895 * t12595 + t12601 - t12604 - t12608 - F::cast_from(0.1025389702100779493e4_f64) * t839 * t12614 + t12620 + t12624 - F::cast_from(0.56969282336565386482e-3_f64) * t834 * t12626;
    (t12624, t12626, t12629)
}
