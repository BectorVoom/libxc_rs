//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 895/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk895<F: Float>(t1828: F, t482: F, t372: F, t371: F, t1715: F, t33426: F, t32015: F, t2142: F, t8190: F) -> (F, F, F, F, F) {
    let t34899 = t482 * t1828;
    let t34900 = t372 * t34899;
    let t34901 = t371 * t34900;
    let t34904 = t33426 * t1715;
    let t34905 = t32015 * t34904;
    let t34908 = t2142 * t8190;
    (t34899, t34901, t34904, t34905, t34908)
}
