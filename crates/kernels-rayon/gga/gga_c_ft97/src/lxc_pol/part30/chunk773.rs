//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 773/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk773(t33649: f64, t33690: f64, t33741: f64, t33787: f64, t7485: f64, t771: f64, t193: f64, t1403: f64, t1427: f64, t247: f64, t33537: f64, t33568: f64, t33573: f64, t33575: f64, t33584: f64, t33589: f64, t33592: f64, t33594: f64, t33596: f64, t33599: f64, t33602: f64, t33608: f64, t33650: f64, t33686: f64, t5996: f64, t6002: f64, t719: f64, t7443: f64, t7487: f64, t7558: f64) -> (f64, f64, f64, f64) {
    let t33789 = t33649 + t33690 + t33741 + t33787;
    let t33792 = t7485 * t771;
    let t33793 = t193 * t33792;
    let t33799 = -t6002 * t33537 / 18.0_f64 + t33568 * t1427 / 6.0_f64 - t33573 - t1403 * t33575 / 3.0_f64 - t5996 * t7443 / 3.0_f64 + t5996 * t7487 / 6.0_f64 + t1403 * t33584 / 6.0_f64 - t33589 + t33592 - t33594 + 4.0_f64 * t33596 - 12.0_f64 * t33599 + 8.0_f64 * t33602 - t247 * t33789 + 8.0_f64 * t33608 + t1403 * t33793 / 6.0_f64 - t719 * t7558 - 2.0_f64 * t33650 + 4.0_f64 * t33686;
    (t33789, t33792, t33793, t33799)
}
