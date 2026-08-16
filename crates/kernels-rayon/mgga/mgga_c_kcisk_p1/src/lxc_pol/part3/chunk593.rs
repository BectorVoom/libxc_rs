//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 593/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk593(t604: f64, t5031: f64, t5032: f64, t1310: f64, t4794: f64, t1783: f64, t1773: f64, t1778: f64, t1787: f64, t4984: f64, t4987: f64, t4989: f64, t4997: f64, t5000: f64, t5003: f64, t5009: f64, t5013: f64, t5017: f64, t5022: f64, t5026: f64, t664: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t659 = 0.0_f64 < t604;
    let t5033 = t5031 * t5032;
    let t5034 = t1310 * t5033;
    let t5038 = piecewise3(t659, t4794, -t4794);
    let t5039 = t1783 * t5038;
    let t5040 = t1310 * t5039;
    let t5043 = 0.5397236614853195164e-1_f64 * t4984 * t664 + 0.35981577432354634426e-1_f64 * t4987 + 0.35981577432354634426e-1_f64 * t4989 * t1778 - 0.10794473229706390328e0_f64 * t4989 * t1787 - t4997 + 0.11993859144118211475e-1_f64 * t5000 - 0.35981577432354634426e-1_f64 * t5003 + 0.23987718288236422951e-1_f64 * t1773 * t5009 - 0.35981577432354634426e-1_f64 * t5013 * t5017 - 0.35981577432354634426e-1_f64 * t1773 * t5022 + 0.17990788716177317213e-1_f64 * t1773 * t5026 + 0.10794473229706390328e0_f64 * t1773 * t5034 - 0.5397236614853195164e-1_f64 * t1773 * t5040;
    (t5033, t5034, t5038, t5039, t5040, t5043)
}
