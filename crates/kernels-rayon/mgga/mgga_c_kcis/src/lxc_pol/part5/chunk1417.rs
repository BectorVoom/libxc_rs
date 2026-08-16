//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1417/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1417(t13031: f64, t18422: f64, t18423: f64, t18424: f64, t18425: f64, t18426: f64, t18427: f64, t20855: f64, t23389: f64, t2654: f64, t4529: f64, t6291: f64, t6297: f64, t6892: f64, t6894: f64, t7573: f64, t8: f64, t8521: f64) -> f64 {
    let t23392 = t6894 + 4.0_f64 * t4529 + t2654 + t13031 - t18422 + t6291 - t18423 - t18424 + t8521 - t7573 - t18425 - t18426 - t6297 + t18427 + t6892 + t8 * (t20855 + t23389);
    t23392
}
