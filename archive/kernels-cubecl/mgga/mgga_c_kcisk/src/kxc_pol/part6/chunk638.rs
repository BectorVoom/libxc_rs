//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 638/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk638<F: Float>(t20: F, t8831: F, t649: F, t1773: F, t2460: F, t2466: F, t4997: F, t5013: F, t664: F, t7206: F, t7208: F, t7216: F, t7219: F, t7231: F, t7254: F, t8794: F, t8798: F, t8802: F, t8807: F, t8811: F, t8816: F, t8822: F, t8825: F) -> (F, F, F) {
    let t8832 = t8831 * t20;
    let t8833 = t649 * t8832;
    let t8845 = F::cast_from(0.5397236614853195164e-1_f64) * t8794 * t664 - F::cast_from(0.35981577432354634426e-1_f64) * t5013 * t8798 - F::cast_from(0.35981577432354634426e-1_f64) * t1773 * t8802 - F::cast_from(0.35981577432354634426e-1_f64) * t7254 + F::cast_from(0.17990788716177317213e-1_f64) * t1773 * t8807 + F::cast_from(0.23987718288236422951e-1_f64) * t1773 * t8811 + F::cast_from(0.10794473229706390328e0_f64) * t1773 * t8816 - t4997 - F::cast_from(0.5397236614853195164e-1_f64) * t1773 * t8822 - F::cast_from(0.28785261945883707542e0_f64) * t8825 * t664 - F::cast_from(0.10794473229706390328e0_f64) * t7208 * t2466 + F::cast_from(0.52772980234120130494e0_f64) * t8833 * t664 + F::cast_from(0.28785261945883707542e0_f64) * t7219 * t2466 + F::cast_from(0.35981577432354634426e-1_f64) * t7206 - F::cast_from(0.95950873152945691804e-1_f64) * t7216 + F::cast_from(0.11993859144118211475e-1_f64) * t7231 + F::cast_from(0.35981577432354634426e-1_f64) * t7208 * t2460 - F::cast_from(0.95950873152945691804e-1_f64) * t7219 * t2460;
    (t8832, t8833, t8845)
}
