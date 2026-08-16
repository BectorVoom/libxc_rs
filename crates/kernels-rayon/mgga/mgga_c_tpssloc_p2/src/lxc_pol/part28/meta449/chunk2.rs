//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1642/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1642(t22819: f64, t22825: f64, t22858: f64, t22863: f64, t22867: f64, t22805: f64, t22809: f64, t22830: f64, t22834: f64, t22837: f64, t22840: f64, t22848: f64, t22850: f64, t22856: f64, t22860: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24049 = 0.33643963411783659044e-4_f64 * t22819;
    let t24050 = 0.10541775202358879834e-2_f64 * t22825;
    let t24058 = 119.0_f64 / 3456.0_f64 * t22858;
    let t24060 = 35.0_f64 / 216.0_f64 * t22863;
    let t24061 = 0.22608743412718618878e-1_f64 * t22867;
    let t24062 = 0.33913115119077928316e-1_f64 * t22805 - 0.24223653656484234512e-2_f64 * t22809 - t24049 + t24050 + 0.48447307312968469024e-2_f64 * t22830 + t22834 / 96.0_f64 + t22837 / 768.0_f64 + t22840 / 8.0_f64 + 0.16956557559538964158e-1_f64 * t22848 + 5.0_f64 / 192.0_f64 * t22850 + 0.13457585364713463618e-3_f64 * t22856 + t24058 - 7.0_f64 / 576.0_f64 * t22860 + t24060 + t24061;
    (t24049, t24050, t24058, t24060, t24061, t24062)
}
