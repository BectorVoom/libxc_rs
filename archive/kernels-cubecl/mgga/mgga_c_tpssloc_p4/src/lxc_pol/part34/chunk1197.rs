//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1197/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1197<F: Float>(t20442: F, t22833: F, t2002: F, t20595: F, t559: F, t1985: F, t26193: F, t28205: F, t28209: F, t6888: F, t20608: F, t6889: F, t80640: F) -> (F, F, F, F, F) {
    let t107198 = t22833 * t20442;
    let t107205 = t20595 * t2002 * t559;
    let t107214 = t1985 * t26193 * t28205;
    let t107230 = t6888 * t26193 * t28209;
    let t107238 = t1985 * t6889 * t80640 * t20608;
    (t107198, t107205, t107214, t107230, t107238)
}
