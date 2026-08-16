//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2815/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2815<F: Float>(t10532: F, t14598: F, t231: F, t50511: F, t2782: F, t2797: F, t10069: F, t14537: F, t1568: F, t2645: F, t2783: F, t1559: F, t40927: F, t40945: F, t40948: F, t40952: F, t40954: F, t40956: F, t40958: F, t820: F) -> F {
    let t51696 = t14598 * t10532;
    let t51698 = t50511 * t231;
    let t51700 = t2782 * t2797 * t51698;
    let t51703 = t10069 * t14537;
    let t51704 = F::cast_from(0.21951497276451705329e-1_f64) * t51703;
    let t51708 = t2782 * t2783 * t1568 * t2645 * t231;
    let t51713 = -F::cast_from(0.13878983423218070566e-1_f64) * t40945 - F::cast_from(0.39029762157531132075e-1_f64) * t40948 + F::cast_from(0.34697458558045176417e-2_f64) * t40952 + F::cast_from(0.39029762157531132075e-2_f64) * t40954 + F::cast_from(0.43902994552903410657e-1_f64) * t40956 + F::cast_from(0.17563392970889009434e0_f64) * t51696 + F::cast_from(0.16463622957338778996e-1_f64) * t51700 - F::cast_from(0.51220160311720645767e-1_f64) * t40958 - t51704 + F::cast_from(0.16463622957338778996e-1_f64) * t51708 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t40927 * t1559;
    t51713
}
