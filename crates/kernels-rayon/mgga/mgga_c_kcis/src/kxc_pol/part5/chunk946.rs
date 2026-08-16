//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 946/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk946(t706: f64, t9220: f64, t2385: f64, t684: f64, t2379: f64, t687: f64, t2390: f64, t688: f64, t707: f64, t8533: f64, t8541: f64, t8753: f64, t8757: f64, t8924: f64, t8926: f64, t8932: f64, t8934: f64, t8937: f64) -> f64 {
    let t9221 = t9220 * t706;
    let t9229 = t684 * t2385;
    let t9232 = t2379 * t687;
    let t9235 = 0.59694999999999999999e-1_f64 * t8533 - 0.59694999999999999999e-1_f64 * t8541 - 0.99491666666666666664e-2_f64 * t8753 - 0.29847499999999999999e-1_f64 * t8757 - 0.66725e-1_f64 * t688 * t9221 + 0.79593333333333333331e-1_f64 * t8924 + 0.39796666666666666665e-1_f64 * t8926 - 0.92858888888888888885e-1_f64 * t8932 - 0.79593333333333333331e-1_f64 * t8934 - 0.29847499999999999999e-1_f64 * t8937 + 0.2671335375e-1_f64 * t9229 * t2390 - 0.200175e0_f64 * t9232 * t707;
    t9235
}
