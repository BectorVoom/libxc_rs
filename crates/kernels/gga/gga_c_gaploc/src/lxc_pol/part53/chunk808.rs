//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 808/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk808<F: Float>(t13158: F, t15766: F, t41425: F, t41430: F, t41435: F, t41445: F, t24549: F, t7584: F, t9438: F, t13072: F, t32757: F, t25359: F, t2615: F, t1445: F, t3209: F, t833: F, t8469: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44099 = 0.21450293971110256001e1 * t15766 * t13158;
    let t44106 = 0.1022478025437886658e1 * t41425;
    let t44110 = 0.19171462976960374838e1 * t41430;
    let t44111 = 0.42603251059911944084e0 * t41435;
    let t44112 = 0.29792074959875355558e-1 * t41445;
    let t44117 = t7584 * t9438 * t24549;
    let t44118 = 0.15976219147466979032e-1 * t44117;
    let t44130 = t32757 * t13072;
    let t44133 = t2615 * t9438 * t25359;
    let t44134 = 0.15976219147466979032e-1 * t44133;
    let t44138 = 0.43710935587469654631e2 * t833 * t1445 * t8469 * t3209;
    (t44099, t44106, t44110, t44111, t44112, t44118, t44130, t44134, t44138)
}
