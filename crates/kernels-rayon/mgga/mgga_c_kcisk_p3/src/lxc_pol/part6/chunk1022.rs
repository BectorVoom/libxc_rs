//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1022/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1022(t30534: f64, t30769: f64, t1328: f64, t2173: f64, t8063: f64, t13440: f64, t1375: f64, t30294: f64, t30273: f64, t457: f64, t158: f64, t173: f64, t25425: f64, t25427: f64, t25429: f64, t25485: f64, t25487: f64, t25489: f64, t25491: f64, t25493: f64) -> (f64, f64, f64) {
    let t30770 = t30534 + t30769;
    let t30771 = t30770 * t1328;
    let t30774 = t8063 * t2173;
    let t30775 = t30774 * t13440;
    let t30787 = t1375 * t30294;
    let t30790 = t1375 * t30273;
    let t30793 = t457 * t30294;
    let t30801 = -0.4684e-2_f64 * t25425 - 0.39624999999999999999e-2_f64 * t25427 + 0.26416666666666666666e-2_f64 * t25429 - 0.2016525e-4_f64 * t173 * t30787 + 0.21078e-1_f64 * t158 * t30790 + 0.3513e-2_f64 * t158 * t30793 + 0.70578375e-4_f64 * t25485 + 0.14052e-1_f64 * t25487 - 0.352891875e-4_f64 * t25489 + 0.4705225e-4_f64 * t25491 - 0.28104e-1_f64 * t25493;
    (t30771, t30775, t30801)
}
