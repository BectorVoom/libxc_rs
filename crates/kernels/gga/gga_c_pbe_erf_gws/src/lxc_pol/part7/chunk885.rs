//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 885/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk885<F: Float>(t18032: F, t550: F, t553: F, t163: F, t169: F, t2019: F, t784: F, t16441: F, t16444: F, t16446: F, t16449: F, t16454: F, t16457: F, t16460: F, t16467: F, t16468: F, t16471: F, t16474: F, t16477: F, t16480: F, t16513: F, t16548: F, t16588: F, t16625: F, t16658: F, t16778: F, t16830: F, t16866: F, t16903: F, t16940: F, t17056: F, t17084: F, t171: F, t17125: F, t17161: F, t17203: F, t17240: F, t17283: F, t17313: F, t17349: F, t17387: F, t17417: F, t17453: F, t17489: F, t17526: F, t17557: F, t17586: F, t17630: F, t17673: F, t17804: F, t17839: F, t18009: F, t18021: F, t18024: F, t18027: F, t18030: F) -> (F,) {
    let t18035 = 0.79015561315637923528e-2 * t550 * t18032 * t553;
    let t18038 = t169 * t784 * t2019 * t163;
    let t18040 = t16441 + t16444 - t16446 + 0.13871971944573393855e-1 * t16449 - t16454 - 0.79015561315637923528e-2 * t16457 - 0.23704668394691377058e-1 * t16460 - t16467 - 0.1035981803916141664e0 * t16468 - t16471 + 0.39507780657818961764e-1 * t16474 + 0.79015561315637923528e-1 * t16477 + t16480 - 0.53884053046145740922e-2 * t169 * t171 * (t17673 + t17526 + t17839 + t17804 + t17489 + t18009 + t17453 + t17349 + t17557 + t17630 + t17586 + t17387 + t16625 + t17417 + t16588 + t17056 + t17313 + t17203 + t17084 + t17125 + t16658 + t16940 + t17240 + t16778 + t17283 + t16903 + t17161 + t16548 + t16830 + t16866 + t16513) * t163 - t18021 - 0.29725654166942986832e-2 * t18024 - 0.59451308333885973663e-2 * t18027 - 0.23704668394691377058e-1 * t18030 - t18035 - 0.14369080812305530913e0 * t18038;
    (t18040,)
}
