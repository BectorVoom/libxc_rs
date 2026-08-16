//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 805/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk805(t717: f64, t780: f64, t1138: f64, t1597: f64, t1124: f64, t483: f64, t485: f64, t1904: f64, t473: f64, t1131: f64, t1910: f64, t4168: f64, t4172: f64, t4175: f64, t4246: f64, t4254: f64, t4260: f64, t4261: f64, t4265: f64, t4268: f64, t4270: f64, t4272: f64, t4275: f64, t4276: f64, t4279: f64) -> (f64, f64, f64, f64) {
    let t5466 = t717 * t780;
    let t5468 = t5466 * t1138 * t1597;
    let t5470 = t1124 * t780;
    let t5472 = t5470 * t483 * t485;
    let t5474 = t473 * t1904;
    let t5477 = 0.003950778065781896_f64 * t5474 * t483 * t485;
    let t5479 = t1910 * t1131 * t485;
    let t5487 = 0.013169260219272987_f64 * t4168 + t4172 + t4175 - 0.0004954275694490498_f64 * t5468 + 0.006584630109636494_f64 * t5472 - t5477 - 0.003950778065781896_f64 * t5479 - 0.12602162889256446_f64 * t4276 - t4279 + t4254 - t4260 - 0.06301081444628223_f64 * t4261 - t4265 - 0.031505407223141116_f64 * t4268 + 0.031505407223141116_f64 * t4270 + 0.12602162889256446_f64 * t4272 + t4275 + 0.008980675507690957_f64 * t4246;
    (t5466, t5470, t5474, t5487)
}
