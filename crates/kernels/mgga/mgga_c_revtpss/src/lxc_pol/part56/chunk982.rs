//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 982/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk982<F: Float>(t33384: F, t33552: F, t3: F, t1461: F, t32365: F, t32368: F, t32371: F, t32373: F, t32377: F, t32901: F, t32903: F, t32905: F, t573: F, t8616: F, t8975: F) -> (F, F, F, F) {
    let t33553 = t33384 + t33552;
    let t33554 = t3 * t33553;
    let t33565 = param_d * t33553;
    let t33572 = F::new(3.0) * t1461 * t8975 + t33565 * t573 + t32365 + t32368 + t32371 + t32373 + t32377 + F::new(6.0) * t32901 + F::new(12.0) * t32903 + F::new(6.0) * t32905 + t8616;
    (t33553, t33554, t33565, t33572)
}
