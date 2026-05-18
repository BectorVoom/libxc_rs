//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 943/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk943<F: Float>(t22857: F, t9994: F, t1390: F, t828: F, t4003: F, t1370: F, t13798: F, t13801: F, t1410: F, t22038: F, t22044: F, t22057: F, t22059: F, t22063: F, t22069: F, t22843: F, t22849: F, t22854: F, t4002: F, t5671: F, t9735: F, t9993: F) -> (F, F, F, F, F) {
    let t22858 = t22857 * t9994;
    let t22860 = t1390 * t828 * t22858;
    let t22863 = t22857 * t4003;
    let t22865 = t1390 * t828 * t22863;
    let t22874 = -F::new(0.51448821741683684367e-2) * t5671 * t22843 + F::new(7.0) / F::new(48.0) * t22038 - F::new(7.0) / F::new(16.0) * t22044 - t1370 * t22849 / F::new(48.0) - t9735 + F::new(0.12862205435420921092e-1) * t1410 * t22854 - F::new(0.12862205435420921092e-2) * t9993 * t22860 + F::new(0.12862205435420921092e-2) * t4002 * t22865 - F::new(0.15246000842785598468e-3) * t22057 - F::new(0.60023625365297631762e-1) * t22059 + F::new(0.21437009059034868486e-4) * t22063 + F::new(0.76230004213927992338e-3) * t22069 - F::new(35.0) / F::new(72.0) * t13798 + F::new(0.30492001685571196935e-4) * t13801;
    (t22858, t22860, t22863, t22865, t22874)
}
