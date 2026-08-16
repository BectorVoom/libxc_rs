//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 773/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk773(t2089: f64, t331: f64, t1268: f64, t4624: f64, t3516: f64, t4610: f64, t4620: f64, t4637: f64, t538: f64, t4633: f64, t4602: f64, t504: f64, t537: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5096 = 0.002962962962962963_f64 * t331 * t2089;
    let t5097 = t1268 * t4624;
    let t5100 = t3516 * t4610;
    let t5103 = t1268 * t4620;
    let t5106 = t538 * t4637;
    let t5109 = t538 * t4633;
    let t5112 = 0.015996296296296297_f64 * t4602;
    let t5121 = t537 * t504;
    (t5096, t5097, t5100, t5103, t5106, t5109, t5112, t5121)
}
