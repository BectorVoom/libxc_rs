//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2944/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2944<F: Float>(t291: F, t78132: F, t78151: F, t15400: F, t1622: F, t1634: F, t19173: F, t19227: F, t19300: F, t23755: F, t23776: F, t2938: F, t41662: F, t4647: F, t4670: F, t52430: F, t6174: F, t64055: F, t64120: F, t77886: F, t77898: F, t77911: F, t77923: F, t77935: F, t77947: F, t77961: F, t77974: F, t78094: F, t78096: F, t78099: F, t78108: F, t78111: F, t946: F, t954: F, t955: F, t974: F) -> (F, F) {
    let t78154 = F::cast_from(0.621814e-1_f64) * (t78132 + t78151) * t291;
    let t78155 = F::cast_from(1.0_f64) * t2938 * t23755 + F::cast_from(1.0_f64) * t946 * (t77886 + t77898 + t77911 + t77923 + t77935 + t77947 + t77961 + t77974) * t954 + F::cast_from(0.2069040516770936012e4_f64) * t41662 * t23776 + F::cast_from(0.17544670867903938621e1_f64) * t64120 * t1634 + F::cast_from(0.10526802520742363173e2_f64) * t52430 * t19300 - t78094 - t78096 - t78099 + F::cast_from(3.0_f64) * t64055 * t1622 + F::cast_from(3.0_f64) * t19173 * t4670 + F::cast_from(3.0_f64) * t15400 * t6174 + F::cast_from(3.0_f64) * t4647 * t19227 + F::cast_from(1.0_f64) * t78108 * t955 + F::cast_from(0.5848223622634646207e0_f64) * t78111 * t974 + t78154;
    (t78154, t78155)
}
