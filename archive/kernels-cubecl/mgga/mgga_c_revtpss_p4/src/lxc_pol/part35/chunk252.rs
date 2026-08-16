//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 252/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk252<F: Float>(t127: F, t371: F, t373: F, t367: F, t365: F, t369: F, t361: F) -> (F, F, F) {
    let t1058 = t371 * t127 * t373;
    let t1060 = F::cast_from(0.14291339372689912324e-3_f64) * t367 * t1058;
    let t1061 = t365 * t369;
    let t1062 = t361 * t1061;
    (t1058, t1060, t1062)
}
