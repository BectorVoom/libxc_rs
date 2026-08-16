//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2844/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2844(t23148: f64, t236: f64, t807: f64, t854: f64, t1559: f64, t18599: f64, t2661: f64, t2662: f64, t40862: f64, t51099: f64, t51100: f64, t51102: f64, t51104: f64, t51122: f64, t51170: f64, t62216: f64, t62236: f64, t62241: f64, t62246: f64, t62251: f64, t62392: f64, t62399: f64, t62401: f64, t62405: f64) -> f64 {
    let t76878 = t807 * t236 * t854 * t23148;
    let t76882 = t2661 * t2662 * t18599 * t1559;
    let t76884 = 0.15246000842785598467e-3_f64 * t62216 - t51099 - 0.38538502130374707237e-2_f64 * t51100 + 0.91464571985215438872e-3_f64 * t51102 + 0.11337795902333997111e0_f64 * t51104 - 0.76230004213927992336e-4_f64 * t62236 - 0.38115002106963996168e-4_f64 * t62241 + t51122 + 0.34299214494455789578e-3_f64 * t62246 - 0.54214778996945588152e-4_f64 * t62251 + 455.0_f64 / 648.0_f64 * t40862 - 0.38115002106963996168e-4_f64 * t62392 - 0.17006693853500995666e-1_f64 * t62399 + 0.34013387707001991332e-1_f64 * t62401 + 0.7623000421392799234e-3_f64 * t62405 + 0.86700792194318801432e-2_f64 * t51170 + 0.28582678745379824648e-4_f64 * t76878 + 0.42874018118069736973e-3_f64 * t76882;
    t76884
}
