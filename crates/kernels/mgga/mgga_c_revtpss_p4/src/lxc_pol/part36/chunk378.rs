//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 378/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk378<F: Float>(t1210: F, t1274: F, t1770: F, t1775: F, t1813: F, t1829: F, t460: F, t495: F) -> F {
    let t1832 = F::cast_from(0.65854491829355115987e0_f64) * t1770 * t495 - F::cast_from(0.65854491829355115987e0_f64) * t1210 * t1775 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t1813 - F::cast_from(0.65854491829355115987e0_f64) * t1274 * t1829;
    t1832
}
