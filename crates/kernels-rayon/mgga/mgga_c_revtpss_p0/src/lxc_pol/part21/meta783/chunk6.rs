//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2815/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2815(t10532: f64, t14598: f64, t231: f64, t50511: f64, t2782: f64, t2797: f64, t10069: f64, t14537: f64, t1568: f64, t2645: f64, t2783: f64, t1559: f64, t40927: f64, t40945: f64, t40948: f64, t40952: f64, t40954: f64, t40956: f64, t40958: f64, t820: f64) -> f64 {
    let t51696 = t14598 * t10532;
    let t51698 = t50511 * t231;
    let t51700 = t2782 * t2797 * t51698;
    let t51703 = t10069 * t14537;
    let t51704 = 0.21951497276451705329e-1_f64 * t51703;
    let t51708 = t2782 * t2783 * t1568 * t2645 * t231;
    let t51713 = -0.13878983423218070566e-1_f64 * t40945 - 0.39029762157531132075e-1_f64 * t40948 + 0.34697458558045176417e-2_f64 * t40952 + 0.39029762157531132075e-2_f64 * t40954 + 0.43902994552903410657e-1_f64 * t40956 + 0.17563392970889009434e0_f64 * t51696 + 0.16463622957338778996e-1_f64 * t51700 - 0.51220160311720645767e-1_f64 * t40958 - t51704 + 0.16463622957338778996e-1_f64 * t51708 - 0.65854491829355115987e0_f64 * t820 * t40927 * t1559;
    t51713
}
