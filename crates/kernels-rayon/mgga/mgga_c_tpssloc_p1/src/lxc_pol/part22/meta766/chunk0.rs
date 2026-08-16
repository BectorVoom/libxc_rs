//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2588/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2588(t22222: f64, t3411: f64, t14858: f64, t6106: f64, t1164: f64, t18275: f64, t21906: f64, t44154: f64, t21830: f64, t6098: f64, t22237: f64, t71876: f64, t71879: f64, t72098: f64, t72104: f64, t72106: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t72201 = 0.35089341735807877242e1_f64 * t3411 * t22222;
    let t72203 = 0.51947577317044391276e2_f64 * t14858 * t6106;
    let t72207 = 0.12304822629859687989e5_f64 * t1164 * t44154 * t21906 * t18275;
    let t72209 = 0.51947577317044391277e2_f64 * t3411 * t21830;
    let t72211 = 0.35089341735807877242e1_f64 * t14858 * t6098;
    let t72213 = 0.10254018858216406658e4_f64 * t3411 * t22237;
    let t72214 = -t72098 - t71876 + t71879 - t72104 - t72106 - t72201 - t72203 + t72207 - t72209 + t72211 - t72213;
    (t72201, t72203, t72207, t72209, t72211, t72213, t72214)
}
