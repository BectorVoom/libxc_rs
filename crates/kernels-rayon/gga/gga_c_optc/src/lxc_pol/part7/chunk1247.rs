//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1247/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1247(t3843: f64, t940: f64, t942: f64, t1: f64, t23951: f64, t24521: f64, t24566: f64, t24575: f64, t24615: f64, t24620: f64, t25059: f64, t25095: f64, t25562: f64, t25730: f64, t25740: f64, t25742: f64, t25751: f64, t25753: f64, t25769: f64, t2644: f64, t2648: f64, t2758: f64, t297: f64, t313: f64, t894: f64, t914: f64, t930: f64, t935: f64, t953: f64) -> f64 {
    let t25772 = t940 * t3843 * t942;
    let t25774 = 0.59710464543246456043e-1_f64 * t25730 + 0.30050434779516693818e0_f64 * t930 * t914 * t24620 + 0.28977204965962526182e-1_f64 * t930 * t914 * t24615 + 0.30228422675018518373e0_f64 * t953 * t25095 + 0.20606012420240018619e0_f64 * t25740 + 0.75587607063262836759e5_f64 * t25742 * t25562 * t935 * t2644 - 0.30228422675018518374e-1_f64 * t953 * t25059 - 0.61944912485988186948e2_f64 * t25751 - 0.67174272611152263053e-2_f64 * t25753 - 0.40304563566691357832e-1_f64 * t953 * t894 * t2648 * t23951 - 0.27821325036192187983e8_f64 * t24566 * t313 * t24575 * t935 - 0.23229342182245570105e2_f64 * t2758 * t313 * t24521 * t1 * t297 + 0.10324152080998031158e2_f64 * t25769 + 0.69310201356862480534e1_f64 * t25772;
    t25774
}
