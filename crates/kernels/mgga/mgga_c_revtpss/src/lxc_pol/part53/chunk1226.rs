//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1226/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1226<F: Float>(t127313: F, t127314: F, t127318: F, t127324: F, t127326: F, t127328: F, t127330: F, t127332: F, t127335: F, t32107: F, t32109: F, t32112: F, t8463: F) -> F {
    let t129426 = t127313 + t127314 + t127318 - F::new(2.0) * t127324 - F::new(2.0) * t127326 - F::new(2.0) * t127328 - F::new(2.0) * t127330 + t127332 - t8463 + t127335 - t32107 - t32109 - t32112;
    t129426
}
