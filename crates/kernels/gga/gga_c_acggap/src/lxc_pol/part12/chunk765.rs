//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 765/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk765<F: Float>(t7631: F, t7638: F, t7640: F, t7671: F, t7673: F, t7622: F, t7624: F, t7626: F, t7628: F, t7644: F, t7648: F, t7650: F, t7652: F, t7654: F, t7659: F, t7661: F, t7663: F, t7665: F, t7667: F) -> (F, F, F, F, F, F) {
    let t8219 = F::cast_from(0.37737710747524982482e-2_f64) * t7631;
    let t8220 = F::cast_from(0.27953859812981468505e-2_f64) * t7638;
    let t8221 = F::cast_from(0.25724410870841842184e-2_f64) * t7640;
    let t8232 = F::cast_from(0.42874018118069736972e-3_f64) * t7671;
    let t8233 = F::new(13.0) / F::new(144.0) * t7673;
    let t8234 = F::cast_from(0.16006300097412701803e-1_f64) * t7622 - F::cast_from(0.68598428988911579156e-2_f64) * t7624 - F::cast_from(0.34299214494455789578e-2_f64) * t7626 + F::cast_from(0.34299214494455789578e-2_f64) * t7628 + t8219 + t8220 - t8221 + F::cast_from(0.21437009059034868486e-2_f64) * t7644 + F::cast_from(0.17149607247227894789e-2_f64) * t7648 + F::cast_from(0.68598428988911579156e-2_f64) * t7650 - F::cast_from(0.34299214494455789578e-2_f64) * t7652 + F::cast_from(0.34299214494455789578e-2_f64) * t7654 - F::cast_from(0.94344276868812456204e-2_f64) * t7659 - F::cast_from(0.68598428988911579156e-2_f64) * t7661 - F::cast_from(0.13719685797782315831e-1_f64) * t7663 + F::cast_from(0.13719685797782315831e-1_f64) * t7665 - F::cast_from(0.85748036236139473944e-3_f64) * t7667 + t8232 - t8233;
    (t8219, t8220, t8221, t8232, t8233, t8234)
}
