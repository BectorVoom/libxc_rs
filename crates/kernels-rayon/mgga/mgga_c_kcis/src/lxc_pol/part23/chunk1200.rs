//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1200/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1200(t28314: f64, t28317: f64, t28320: f64, t28323: f64, t27722: f64, t28901: f64, t91769: f64, t91772: f64, t91773: f64, t91776: f64, t91777: f64, t91778: f64, t91781: f64, t91785: f64, t95271: f64, t95275: f64, t97622: f64) -> f64 {
    let t97623 = t28314 / 8.0_f64;
    let t97624 = t28317 / 8.0_f64;
    let t97625 = t28320 / 8.0_f64;
    let t97626 = t28323 / 8.0_f64;
    let t97627 = -t91769 + t91772 + t91773 + t97622 + t95271 - t91776 - t97623 + t91777 + t28901 - t91778 + t95275 - t97624 - t91781 + t97625 - t91785 - t97626 - t27722;
    t97627
}
