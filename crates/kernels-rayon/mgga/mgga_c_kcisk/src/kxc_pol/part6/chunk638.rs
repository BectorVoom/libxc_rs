//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 638/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk638(t20: f64, t8831: f64, t649: f64, t1773: f64, t2460: f64, t2466: f64, t4997: f64, t5013: f64, t664: f64, t7206: f64, t7208: f64, t7216: f64, t7219: f64, t7231: f64, t7254: f64, t8794: f64, t8798: f64, t8802: f64, t8807: f64, t8811: f64, t8816: f64, t8822: f64, t8825: f64) -> (f64, f64, f64) {
    let t8832 = t8831 * t20;
    let t8833 = t649 * t8832;
    let t8845 = 0.5397236614853195164e-1_f64 * t8794 * t664 - 0.35981577432354634426e-1_f64 * t5013 * t8798 - 0.35981577432354634426e-1_f64 * t1773 * t8802 - 0.35981577432354634426e-1_f64 * t7254 + 0.17990788716177317213e-1_f64 * t1773 * t8807 + 0.23987718288236422951e-1_f64 * t1773 * t8811 + 0.10794473229706390328e0_f64 * t1773 * t8816 - t4997 - 0.5397236614853195164e-1_f64 * t1773 * t8822 - 0.28785261945883707542e0_f64 * t8825 * t664 - 0.10794473229706390328e0_f64 * t7208 * t2466 + 0.52772980234120130494e0_f64 * t8833 * t664 + 0.28785261945883707542e0_f64 * t7219 * t2466 + 0.35981577432354634426e-1_f64 * t7206 - 0.95950873152945691804e-1_f64 * t7216 + 0.11993859144118211475e-1_f64 * t7231 + 0.35981577432354634426e-1_f64 * t7208 * t2460 - 0.95950873152945691804e-1_f64 * t7219 * t2460;
    (t8832, t8833, t8845)
}
