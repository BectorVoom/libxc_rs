//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2224/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2224(t225: f64, t25820: f64, t23384: f64, t25827: f64, t25436: f64, t23328: f64, t23394: f64, t10170: f64, t1049: f64, t1050: f64, t1066: f64, t13735: f64, t13743: f64, t14549: f64, t14659: f64, t1634: f64, t1635: f64, t1956: f64, t23327: f64, t23331: f64, t254: f64, t25712: f64, t25759: f64, t343: f64, t50703: f64, t6687: f64, t6690: f64, t6704: f64, t6771: f64, t7625: f64, t82481: f64, t83276: f64, t83281: f64, t883: f64) -> f64 {
    let t88744 = t25820 * t225;
    let t88753 = 0.54831135561607547884e-2_f64 * t23384 * t25827;
    let t88758 = 0.18277045187202515961e-2_f64 * t23384 * t25436;
    let t88772 = t23328 * t23394;
    let t88779 = -2.0_f64 * t88744 * t1066 - t6771 * t14659 - 0.49348022005446793095e-1_f64 * t6687 * t6704 * t82481 * t13735 - t88753 - t50703 * t1956 + 2.0_f64 * t6771 * t14549 + t88758 - 0.16449340668482264365e-1_f64 * t6687 * t25712 * t343 * t1049 * t6690 - 12.0_f64 * t1050 * t254 * t25759 + 4.0_f64 * t6771 * t13743 - t10170 * t7625 - 2.0_f64 * t83276 * t1635 + 0.10966227112321509577e-1_f64 * t23327 * t88772 * t1634 * t883 * t23331 - 0.12184696791468343974e-2_f64 * t83281;
    t88779
}
