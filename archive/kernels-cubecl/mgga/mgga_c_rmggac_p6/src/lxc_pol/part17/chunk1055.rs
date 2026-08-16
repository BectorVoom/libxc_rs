//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1055/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1055<F: Float>(t1502: F, t16503: F, t34976: F, t571: F, t40771: F, t9147: F, t10066: F, t34764: F, t1685: F, t2405: F, t40750: F, t41571: F, t41579: F, t41651: F, t47295: F, t47302: F, t47306: F, t47310: F, t47312: F, t47316: F, t47321: F, t47325: F, t47327: F, t4965: F, t530: F, t72: F, t9852: F) -> F {
    let t47331 = t16503 * t34976 * t571 * t1502;
    let t47333 = t40771 * t9147;
    let t47335 = t34764 * t10066;
    let t47338 = t40750 - F::cast_from(0.2993560425465952141e-1_f64) * t47295 - F::cast_from(0.4726e1_f64) * t530 * t41651 + F::cast_from(2.0_f64) * t72 * t1685 * t2405 - F::cast_from(0.42564599893297839398e-5_f64) * t47302 + F::cast_from(0.42564599893297839398e-5_f64) * t47306 - F::cast_from(0.38906704589967556326e-4_f64) * t47310 - F::cast_from(0.4726e1_f64) * t47312 - F::cast_from(0.11974241701863808564e0_f64) * t4965 * t9852 + F::cast_from(0.14967802127329760705e-1_f64) * t47316 + F::cast_from(2.0_f64) * t41571 - F::cast_from(0.25538759935978703639e-4_f64) * t47321 - F::cast_from(0.17025839957319135759e-4_f64) * t47325 + F::cast_from(0.85129199786595678796e-5_f64) * t47327 + F::cast_from(0.85129199786595678796e-5_f64) * t47331 + F::cast_from(0.1064114997332445985e-4_f64) * t47333 + F::cast_from(0.1064114997332445985e-4_f64) * t47335 + F::cast_from(0.74488049813271218946e-4_f64) * t41579;
    t47338
}
