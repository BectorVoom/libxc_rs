//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1192/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1192<F: Float>(t1579: F, t4533: F, t2770: F, t212: F, t6041: F, t780: F, t689: F, t10498: F, t10501: F, t14474: F, t14479: F, t14484: F, t14486: F, t14985: F, t14989: F, t14992: F, t14995: F, t865: F) -> F {
    let t18312 = t1579 * t4533;
    let t18313 = t2770 * t18312;
    let t18316 = t212 * t6041;
    let t18317 = t18316 * t780;
    let t18318 = t689 * t18317;
    let t18322 = -F::cast_from(0.13009920719177044025e-2_f64) * t14474 - t14479 - t14484 + F::cast_from(0.26019841438354088051e-1_f64) * t14486 + F::cast_from(0.26341796731742046394e1_f64) * t865 * t18313 - F::cast_from(0.54878743191129263322e-2_f64) * t18318 - t14985 - t14989 + F::cast_from(0.39029762157531132076e-1_f64) * t14992 - t14995 + F::cast_from(0.73171657588172351096e-2_f64) * t10498 + t10501;
    t18322
}
