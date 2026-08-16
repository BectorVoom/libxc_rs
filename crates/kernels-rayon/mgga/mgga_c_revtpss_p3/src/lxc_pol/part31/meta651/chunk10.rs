//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2162/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2162(t107225: f64, t11249: f64, t4746: f64, t7810: f64, t29834: f64, t7143: f64, t1000: f64, t100698: f64, t100737: f64, t1071: f64, t1096: f64, t19452: f64, t1985: f64, t1986: f64, t25476: f64, t27441: f64, t27609: f64, t27696: f64, t29740: f64, t29748: f64, t29843: f64, t29875: f64, t4742: f64, t6250: f64, t7144: f64, t7145: f64, t7160: f64, t7162: f64, t93498: f64, t93502: f64, t93921: f64, t94042: f64, t988: f64, t99743: f64, t99824: f64, t999: f64) -> (f64, f64) {
    let t107268 = t107225 * t11249;
    let t107283 = t4746 * t7810;
    let t107286 = t29834 * t7143;
    let t107305 = -0.17347256376410398924e1_f64 * t25476 * t29748 - 0.17347256376410398924e1_f64 * t7144 * t7145 * t7810 * t4742 + 0.34694512752820797848e1_f64 * t93502 * t29843 * t93498 - 0.4336814094102599731e0_f64 * t99743 * t107268 * t19452 - 0.17347256376410398924e1_f64 * t100737 * t29740 - 0.17347256376410398924e1_f64 * t94042 * t29740 - 0.8673628188205199462e0_f64 * t29834 * t1071 * t1986 + 0.17347256376410398924e1_f64 * t27609 * t27441 - 0.52041769129231196772e1_f64 * t27609 * t27696 - 0.13170898365871023197e1_f64 * t107283 * t1000 + 0.17347256376410398924e1_f64 * t107286 * t7162 + 0.17347256376410398924e1_f64 * t7144 * t7160 * t29875 * t1096 - 0.69389025505641595696e1_f64 * t93921 * t1985 * t6250 * t988 + 0.10408353825846239354e2_f64 * t99824 * t1985 * t6250 * t999 + 0.10408353825846239354e2_f64 * t100698 * t1985 * t6250 * t1096;
    (t107268, t107305)
}
