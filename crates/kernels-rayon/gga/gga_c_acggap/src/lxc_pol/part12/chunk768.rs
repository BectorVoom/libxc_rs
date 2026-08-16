//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 768/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk768(t7805: f64, t7849: f64, t7853: f64, t7862: f64, t7809: f64, t7813: f64, t7817: f64, t7820: f64, t7823: f64, t7825: f64, t7829: f64, t7833: f64, t7837: f64, t7840: f64, t7845: f64, t7847: f64, t7856: f64, t7864: f64, t7868: f64, t7872: f64) -> (f64, f64, f64, f64, f64) {
    let t8278 = 0.41930789719472202758e-3_f64 * t7805;
    let t8291 = 77.0_f64 / 864.0_f64 * t7849;
    let t8292 = 35.0_f64 / 216.0_f64 * t7853;
    let t8294 = t7862 / 192.0_f64;
    let t8298 = -t8278 + 0.22921875e-1_f64 * t7809 + 0.1528125e-1_f64 * t7813 + t7817 / 32.0_f64 + 0.4584375e-1_f64 * t7820 - 0.34299214494455789578e-2_f64 * t7823 + 0.34299214494455789578e-2_f64 * t7825 - t7829 / 64.0_f64 + 0.31448092289604152069e-3_f64 * t7833 + 0.42874018118069736972e-3_f64 * t7837 + 0.62896184579208304138e-3_f64 * t7840 + 0.41930789719472202758e-3_f64 * t7845 - 0.42874018118069736972e-3_f64 * t7847 + t8291 + t8292 + t7856 / 48.0_f64 + t8294 - 7.0_f64 / 72.0_f64 * t7864 + 0.62896184579208304137e-2_f64 * t7868 - 0.94344276868812456206e-2_f64 * t7872;
    (t8278, t8291, t8292, t8294, t8298)
}
