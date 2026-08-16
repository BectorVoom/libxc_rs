//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1223/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1223(t101708: f64, t105621: f64, t105629: f64, t105634: f64, t105638: f64, t105642: f64, t105646: f64, t1510: f64, t2051: f64, t20870: f64, t20937: f64, t26661: f64, t29000: f64, t29052: f64, t4166: f64, t4291: f64, t5585: f64, t5617: f64, t7101: f64, t812: f64, t87177: f64, t92521: f64, t98490: f64, t98505: f64, t98516: f64, t98549: f64) -> f64 {
    let t108218 = -0.23029076935875170111e0_f64 * t98490 + t20937 * t2051 + 0.23029076935875170111e0_f64 * t98505 + 6.0_f64 * t4166 * t29000 + 6.0_f64 * t812 * t92521 * t5585 - 0.14804406601634037928e0_f64 * t98516 - 0.16449340668482264365e-1_f64 * t105621 + 0.49348022005446793095e-1_f64 * t87177 - 0.9869604401089358619e-1_f64 * t105629 - 3.0_f64 * t4291 * t101708 * t1510 - 3.0_f64 * t812 * t26661 * t5617 + 0.49348022005446793095e-1_f64 * t98549 + 0.9869604401089358619e-1_f64 * t105634 - 0.16449340668482264365e-1_f64 * t105638 - 0.49348022005446793095e-1_f64 * t105642 + 0.29608813203268075857e0_f64 * t105646 - 3.0_f64 * t4166 * t29052 - t812 * t7101 * t20870;
    t108218
}
