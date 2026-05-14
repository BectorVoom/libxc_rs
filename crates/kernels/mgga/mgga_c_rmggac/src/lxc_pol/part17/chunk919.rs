//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 919/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk919<F: Float>(t16503: F, t3369: F, t571: F, t8430: F, t1357: F, t34976: F, t8435: F, t10030: F, t34761: F, t1502: F, t40771: F, t9147: F, t10066: F, t34764: F, t1685: F, t2405: F, t40750: F, t41571: F, t41579: F, t41651: F, t47295: F, t47302: F, t47306: F, t47310: F, t47312: F, t47316: F, t4965: F, t530: F, t72: F, t9852: F) -> (F,) {
    let t47321 = t16503 * t3369 * t571 * t8430;
    let t47325 = t16503 * t34976 * t1357 * t8435;
    let t47327 = t34761 * t10030;
    let t47331 = t16503 * t34976 * t571 * t1502;
    let t47333 = t40771 * t9147;
    let t47335 = t34764 * t10066;
    let t47338 = t40750 - 0.2993560425465952141e-1 * t47295 - 0.4726e1 * t530 * t41651 + 2.0 * t72 * t1685 * t2405 - 0.42564599893297839398e-5 * t47302 + 0.42564599893297839398e-5 * t47306 - 0.38906704589967556326e-4 * t47310 - 0.4726e1 * t47312 - 0.11974241701863808564e0 * t4965 * t9852 + 0.14967802127329760705e-1 * t47316 + 2.0 * t41571 - 0.25538759935978703639e-4 * t47321 - 0.17025839957319135759e-4 * t47325 + 0.85129199786595678796e-5 * t47327 + 0.85129199786595678796e-5 * t47331 + 0.1064114997332445985e-4 * t47333 + 0.1064114997332445985e-4 * t47335 + 0.74488049813271218946e-4 * t41579;
    (t47338,)
}
