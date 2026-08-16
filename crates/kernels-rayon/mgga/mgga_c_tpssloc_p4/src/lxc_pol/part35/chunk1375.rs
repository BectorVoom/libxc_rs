//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1375/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1375(t105419: f64, t105621: f64, t105629: f64, t105634: f64, t105638: f64, t105642: f64, t105646: f64, t1510: f64, t16673: f64, t1909: f64, t20937: f64, t20986: f64, t226: f64, t235: f64, t25261: f64, t4281: f64, t4291: f64, t5612: f64, t7533: f64, t812: f64, t87177: f64, t98488: f64, t98490: f64, t98505: f64, t98516: f64, t98549: f64, t98592: f64) -> f64 {
    let t105650 = t20937 * t1909 + 0.57572692339687925277e-1_f64 * t98488 - 0.11514538467937585055e0_f64 * t98490 - 3.0_f64 * t4291 * t25261 * t5612 + 0.11514538467937585055e0_f64 * t98505 - 3.0_f64 * t812 * t98592 * t1510 - 3.0_f64 * t16673 * t7533 - 0.74022033008170189643e-1_f64 * t98516 - 0.82246703342411321825e-2_f64 * t105621 + 0.24674011002723396547e-1_f64 * t87177 + 6.0_f64 * t4281 * t25261 * t20986 - 0.49348022005446793095e-1_f64 * t105629 + 0.24674011002723396547e-1_f64 * t98549 + 0.49348022005446793095e-1_f64 * t105634 - 0.82246703342411321825e-2_f64 * t105638 - 0.24674011002723396548e-1_f64 * t105642 + 0.14804406601634037928e0_f64 * t105646 + t226 * t235 * t105419;
    t105650
}
