//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 991/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk991(t2260: f64, t3927: f64, t1432: f64, t2252: f64, t256: f64, t1427: f64, t5795: f64, t5791: f64, t656: f64, t3912: f64, t5798: f64, t3915: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15125 = t2260 * t3927;
    let t15138 = t2252 * t1432 * t256;
    let t15139 = t5795 * t1427;
    let t15140 = 0.36466666666666664_f64 * t15139;
    let t15143 = t5791 * t656;
    let t15144 = 4.0_f64 / 3.0_f64 * t15143;
    let t15145 = t5795 * t3912;
    let t15146 = (2e-21_f64 as f64) * t15145;
    let t15147 = t5798 * t656;
    let t15149 = t2260 * t3915;
    (t15125, t15138, t15140, t15144, t15146, t15147, t15149)
}
