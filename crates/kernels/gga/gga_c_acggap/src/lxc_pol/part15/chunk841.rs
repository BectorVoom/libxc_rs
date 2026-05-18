//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 841/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk841<F: Float>(t8171: F, t8184: F, t8716: F, t8718: F, t8722: F, t8742: F, t8744: F, t9609: F, t9611: F, t9615: F, t9619: F, t9623: F, t9627: F, t9631: F, t9634: F, t9638: F, t9642: F, t9646: F, t9650: F, t9654: F) -> F {
    let t9911 = F::new(0.32012600194825403606e-1) * t8716 - F::new(0.68598428988911579156e-2) * t8718 - F::new(0.25724410870841842184e-2) * t8722 + F::new(0.37737710747524982482e-2) * t9609 + F::new(0.68598428988911579156e-2) * t9611 - t8171 + t9615 / F::new(16.0) + t9619 / F::new(96.0) - t9623 / F::new(64.0) - t9627 / F::new(192.0) - F::new(0.7640625e-2) * t9631 - F::new(0.42874018118069736972e-3) * t9634 - F::new(0.21437009059034868486e-3) * t9638 + F::new(0.31448092289604152069e-3) * t9642 - F::new(0.62896184579208304138e-3) * t9646 + F::new(0.42874018118069736972e-3) * t9650 - F::new(0.94344276868812456206e-2) * t9654 + F::new(0.916875e-1) * t8742 + F::new(0.61125e-1) * t8744 + t8184;
    t9911
}
