//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 716/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk716<F: Float>(t22249: F, t9061: F, t1333: F, t8859: F, t10409: F, t8486: F, t10494: F, t8959: F, t5074: F, t8955: F, t3521: F, t8900: F, t8904: F, t2063: F, t2372: F, t1417: F, t8916: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22250 = t22249 * sigma2;
    let t22254 = t9061 * sigma2;
    let t22265 = t1333 * t8859;
    let t22328 = t10409 * t8486;
    let t22353 = t10494 * t8959;
    let t22355 = t5074 * t8955;
    let t22412 = t3521 * t8900;
    let t22414 = t3521 * t8904;
    let t22417 = t2063 * t2372;
    let t22469 = t1417 * t8916;
    (t22250, t22254, t22265, t22328, t22353, t22355, t22412, t22414, t22417, t22469)
}
