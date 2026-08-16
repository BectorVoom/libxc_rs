//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1055/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1055(t1960: f64, t5511: f64, t157: f64, t1814: f64, t406: f64, t8397: f64, t8423: f64, t33795: f64, t615: f64, t1658: f64, t2146: f64, t2147: f64, t2331: f64, t29992: f64, t30006: f64, t30011: f64, t30015: f64, t33444: f64, t33451: f64, t33459: f64, t33465: f64, t33468: f64, t7912: f64, t7932: f64, t8400: f64, t8403: f64, t9794: f64) -> (f64, f64, f64) {
    let t38644 = t1960 * t5511;
    let t38647 = t1814 * t406 * t157;
    let t38660 = t8397 * t8423;
    let t38662 = t615 * t33795;
    let t38665 = 0.17347256376410398924e1_f64 * t33444 - 0.8673628188205199462e0_f64 * t29992 + 0.65854491829355115987e0_f64 * t38644 + 0.4336814094102599731e0_f64 * t8400 * t7932 * t38647 - 0.34694512752820797848e1_f64 * t33451 + t33459 + 0.17347256376410398924e1_f64 * t33465 + 0.17347256376410398924e1_f64 * t2146 * t2147 * t2331 * t1658 + 0.8673628188205199462e0_f64 * t7912 * t9794 + t33468 + 0.17347256376410398924e1_f64 * t30006 - 0.17347256376410398924e1_f64 * t38660 + t30011 + 0.8673628188205199462e0_f64 * t38662 * t8403 + t30015;
    (t38647, t38662, t38665)
}
