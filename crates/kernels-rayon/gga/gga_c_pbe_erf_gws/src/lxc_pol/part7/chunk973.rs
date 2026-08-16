//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 973/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk973(t1960: f64, t4579: f64, t553: f64, t1: f64, t4576: f64, t550: f64, t163: f64, t169: f64, t2019: f64, t784: f64, t16441: f64, t16444: f64, t16446: f64, t16449: f64, t16454: f64, t16457: f64, t16460: f64, t16467: f64, t16468: f64, t16471: f64, t16474: f64, t16477: f64, t16480: f64, t16513: f64, t16548: f64, t16588: f64, t16625: f64, t16658: f64, t16778: f64, t16830: f64, t16866: f64, t16903: f64, t16940: f64, t17056: f64, t17084: f64, t171: f64, t17125: f64, t17161: f64, t17203: f64, t17240: f64, t17283: f64, t17313: f64, t17349: f64, t17387: f64, t17417: f64, t17453: f64, t17489: f64, t17526: f64, t17557: f64, t17586: f64, t17630: f64, t17673: f64, t17804: f64, t17839: f64, t18009: f64, t18021: f64, t18024: f64, t18027: f64) -> (f64, f64) {
    let t18030 = t1960 * t4579 * t553;
    let t18032 = t4576 * t1;
    let t18035 = 0.79015561315637923528e-2_f64 * t550 * t18032 * t553;
    let t18038 = t169 * t784 * t2019 * t163;
    let t18040 = t16441 + t16444 - t16446 + 0.13871971944573393855e-1_f64 * t16449 - t16454 - 0.79015561315637923528e-2_f64 * t16457 - 0.23704668394691377058e-1_f64 * t16460 - t16467 - 0.1035981803916141664e0_f64 * t16468 - t16471 + 0.39507780657818961764e-1_f64 * t16474 + 0.79015561315637923528e-1_f64 * t16477 + t16480 - 0.53884053046145740922e-2_f64 * t169 * t171 * (t17673 + t17526 + t17839 + t17804 + t17489 + t18009 + t17453 + t17349 + t17557 + t17630 + t17586 + t17387 + t16625 + t17417 + t16588 + t17056 + t17313 + t17203 + t17084 + t17125 + t16658 + t16940 + t17240 + t16778 + t17283 + t16903 + t17161 + t16548 + t16830 + t16866 + t16513) * t163 - t18021 - 0.29725654166942986832e-2_f64 * t18024 - 0.59451308333885973663e-2_f64 * t18027 - 0.23704668394691377058e-1_f64 * t18030 - t18035 - 0.14369080812305530913e0_f64 * t18038;
    (t18032, t18040)
}
