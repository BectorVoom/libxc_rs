//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 841/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk841<F: Float>(t8171: F, t8184: F, t8716: F, t8718: F, t8722: F, t8742: F, t8744: F, t9609: F, t9611: F, t9615: F, t9619: F, t9623: F, t9627: F, t9631: F, t9634: F, t9638: F, t9642: F, t9646: F, t9650: F, t9654: F) -> F {
    let t9911 = F::cast_from(0.32012600194825403606e-1_f64) * t8716 - F::cast_from(0.68598428988911579156e-2_f64) * t8718 - F::cast_from(0.25724410870841842184e-2_f64) * t8722 + F::cast_from(0.37737710747524982482e-2_f64) * t9609 + F::cast_from(0.68598428988911579156e-2_f64) * t9611 - t8171 + t9615 / F::cast_from(16.0_f64) + t9619 / F::cast_from(96.0_f64) - t9623 / F::cast_from(64.0_f64) - t9627 / F::cast_from(192.0_f64) - F::cast_from(0.7640625e-2_f64) * t9631 - F::cast_from(0.42874018118069736972e-3_f64) * t9634 - F::cast_from(0.21437009059034868486e-3_f64) * t9638 + F::cast_from(0.31448092289604152069e-3_f64) * t9642 - F::cast_from(0.62896184579208304138e-3_f64) * t9646 + F::cast_from(0.42874018118069736972e-3_f64) * t9650 - F::cast_from(0.94344276868812456206e-2_f64) * t9654 + F::cast_from(0.916875e-1_f64) * t8742 + F::cast_from(0.61125e-1_f64) * t8744 + t8184;
    t9911
}
