//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 692/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk692(t1155: f64, t164: f64, t1191: f64, t163: f64, t169: f64, t234: f64, t4137: f64, t1590: f64, t466: f64, t2908: f64, t148: f64, t1203: f64, t479: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4254 = 0.1890324433388467_f64 * t1155 * t164;
    let t4258 = 0.0878110494085338_f64 * t169 * t1191 * t234 * t163;
    let t4259 = t4137 * t164;
    let t4260 = 0.00011865309871651405_f64 * t4259;
    let t4261 = t466 * t1590;
    let t4263 = t2908 * t163;
    let t4265 = 0.031505407223141116_f64 * t148 * t4263;
    let t4268 = t1203 * t479;
    (t4254, t4258, t4259, t4260, t4261, t4263, t4265, t4268)
}
