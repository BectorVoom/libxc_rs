//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1333/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1333(t25304: f64, t25949: f64, t25946: f64, t25878: f64, t94661: f64, t7246: f64, t9692: f64, t1444: f64, t25884: f64, t25924: f64, t25930: f64, t25931: f64, t4056: f64, t543: f64, t7274: f64, t7295: f64, t7298: f64, t7301: f64, t94610: f64, t94749: f64, t94752: f64, t94756: f64, t94758: f64, t94761: f64, t94766: f64, t94769: f64, t94772: f64, t94774: f64) -> f64 {
    let t94776 = t25304 * t25949;
    let t94777 = t94776 * t25946;
    let t94779 = t25878 * t94661;
    let t94784 = 0.30356481678079769392e-1_f64 * t7246 * t9692;
    let t94794 = -0.58544643236296698113e-1_f64 * t94749 - 0.26020884564615598386e1_f64 * t25930 * t25931 * t94752 - 0.28912093960683998208e-1_f64 * t94756 + 0.21951497276451705329e-1_f64 * t94758 - t94761 - 0.77108554593144223218e-1_f64 * t94766 + 0.43368140941025997312e-1_f64 * t94769 - 0.10281140612419229763e-1_f64 * t94772 - 0.77108554593144223218e-1_f64 * t94774 - 0.68549505033305214441e-2_f64 * t94777 - 0.10281140612419229762e0_f64 * t94779 + 0.26020884564615598386e1_f64 * t94610 * t7298 + t94784 - 0.78062653693846795158e1_f64 * t7295 * t25924 * t25884 * t1444 + 0.13010442282307799193e1_f64 * t7295 * t7301 * t7274 * t4056 * t543;
    t94794
}
