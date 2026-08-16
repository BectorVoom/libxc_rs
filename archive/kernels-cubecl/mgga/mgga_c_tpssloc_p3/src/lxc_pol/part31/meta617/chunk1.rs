//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1866/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1866<F: Float>(t22674: F, t28205: F, t6897: F, t22892: F, t28209: F, t22666: F, t22685: F, t28191: F, t6888: F, t19631: F, t6889: F, t6890: F) -> (F, F, F, F, F) {
    let t96878 = t6897 * t22674 * t28205;
    let t96893 = t22892 * t22674 * t28209;
    let t96896 = t22685 * t22666 * t28191;
    let t96900 = t6888 * t22666 * t28209;
    let t96905 = t6888 * t6889 * t6890 * t19631;
    (t96878, t96893, t96896, t96900, t96905)
}
