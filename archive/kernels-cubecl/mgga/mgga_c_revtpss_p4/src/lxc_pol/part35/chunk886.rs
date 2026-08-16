//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 886/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk886<F: Float>(t22857: F, t9994: F, t1390: F, t828: F, t4003: F, t1370: F, t13798: F, t13801: F, t1410: F, t22038: F, t22044: F, t22057: F, t22059: F, t22063: F, t22069: F, t22843: F, t22849: F, t22854: F, t4002: F, t5671: F, t9735: F, t9993: F) -> (F, F, F, F, F) {
    let t22858 = t22857 * t9994;
    let t22860 = t1390 * t828 * t22858;
    let t22863 = t22857 * t4003;
    let t22865 = t1390 * t828 * t22863;
    let t22874 = -F::cast_from(0.51448821741683684367e-2_f64) * t5671 * t22843 + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t22038 - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t22044 - t1370 * t22849 / F::cast_from(48.0_f64) - t9735 + F::cast_from(0.12862205435420921092e-1_f64) * t1410 * t22854 - F::cast_from(0.12862205435420921092e-2_f64) * t9993 * t22860 + F::cast_from(0.12862205435420921092e-2_f64) * t4002 * t22865 - F::cast_from(0.15246000842785598468e-3_f64) * t22057 - F::cast_from(0.60023625365297631762e-1_f64) * t22059 + F::cast_from(0.21437009059034868486e-4_f64) * t22063 + F::cast_from(0.76230004213927992338e-3_f64) * t22069 - F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t13798 + F::cast_from(0.30492001685571196935e-4_f64) * t13801;
    (t22858, t22860, t22863, t22865, t22874)
}
