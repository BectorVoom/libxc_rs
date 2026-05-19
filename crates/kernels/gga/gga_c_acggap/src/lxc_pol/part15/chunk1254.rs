//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1254/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1254<F: Float>(t35835: F, t37739: F, t37741: F, t40196: F, t40200: F, t40204: F, t40208: F, t40212: F, t40218: F, t40220: F, t40222: F, t40226: F, t40230: F, t40234: F, t40237: F, t40241: F, t40243: F, t40246: F) -> F {
    let t42006 = F::cast_from(0.62896184579208304137e-2_f64) * t40196 + F::cast_from(0.62896184579208304137e-2_f64) * t40200 + F::cast_from(0.41930789719472202758e-2_f64) * t40204 - F::cast_from(0.21437009059034868486e-3_f64) * t40208 - F::cast_from(0.64311027177104605458e-2_f64) * t40212 + F::cast_from(0.18868855373762491241e-2_f64) * t40218 - F::cast_from(0.31448092289604152068e-2_f64) * t40220 - F::cast_from(0.51448821741683684368e-2_f64) * t35835 + t37739 - F::cast_from(0.12862205435420921092e-2_f64) * t40222 + F::cast_from(0.62896184579208304138e-3_f64) * t40226 + F::cast_from(0.62896184579208304138e-3_f64) * t40230 + F::cast_from(0.31448092289604152069e-3_f64) * t40234 + t37741 + F::cast_from(0.42874018118069736972e-3_f64) * t40237 - F::cast_from(0.94344276868812456207e-3_f64) * t40241 - F::cast_from(0.34299214494455789578e-1_f64) * t40243 + F::cast_from(0.21437009059034868486e-2_f64) * t40246;
    t42006
}
