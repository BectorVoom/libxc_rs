//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 791/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk791<F: Float>(t2068: F, t3462: F, t1871: F, t8664: F, t9061: F, t1333: F, t8859: F, t10409: F, t8486: F, t10494: F, t8959: F, t5074: F, t8955: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t22167 = t2068 * t3462;
    let t22249 = t8664 * t1871;
    let t22250 = t22249 * sigma2;
    let t22254 = t9061 * sigma2;
    let t22265 = t1333 * t8859;
    let t22328 = t10409 * t8486;
    let t22353 = t10494 * t8959;
    let t22355 = t5074 * t8955;
    (t22167, t22249, t22250, t22254, t22265, t22328, t22353, t22355)
}
