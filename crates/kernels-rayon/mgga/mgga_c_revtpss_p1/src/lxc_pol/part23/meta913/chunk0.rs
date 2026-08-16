//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2944/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2944(t291: f64, t78132: f64, t78151: f64, t15400: f64, t1622: f64, t1634: f64, t19173: f64, t19227: f64, t19300: f64, t23755: f64, t23776: f64, t2938: f64, t41662: f64, t4647: f64, t4670: f64, t52430: f64, t6174: f64, t64055: f64, t64120: f64, t77886: f64, t77898: f64, t77911: f64, t77923: f64, t77935: f64, t77947: f64, t77961: f64, t77974: f64, t78094: f64, t78096: f64, t78099: f64, t78108: f64, t78111: f64, t946: f64, t954: f64, t955: f64, t974: f64) -> (f64, f64) {
    let t78154 = 0.621814e-1_f64 * (t78132 + t78151) * t291;
    let t78155 = 1.0_f64 * t2938 * t23755 + 1.0_f64 * t946 * (t77886 + t77898 + t77911 + t77923 + t77935 + t77947 + t77961 + t77974) * t954 + 0.2069040516770936012e4_f64 * t41662 * t23776 + 0.17544670867903938621e1_f64 * t64120 * t1634 + 0.10526802520742363173e2_f64 * t52430 * t19300 - t78094 - t78096 - t78099 + 3.0_f64 * t64055 * t1622 + 3.0_f64 * t19173 * t4670 + 3.0_f64 * t15400 * t6174 + 3.0_f64 * t4647 * t19227 + 1.0_f64 * t78108 * t955 + 0.5848223622634646207e0_f64 * t78111 * t974 + t78154;
    (t78154, t78155)
}
