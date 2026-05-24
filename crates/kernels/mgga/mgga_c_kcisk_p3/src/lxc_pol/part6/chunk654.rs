//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 654/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk654<F: Float>(t1948: F, t9069: F, t5322: F, t8946: F, t5321: F, t2452: F, t651: F, t742: F, t79: F, t747: F, t741: F, t2586: F, t2590: F) -> (F, F, F, F, F, F, F) {
    let t9070 = t1948 * t9069;
    let t9072 = t5322 * t8946;
    let t9073 = t5321 * t9072;
    let t9077 = F::new(1.0) / t651 / t742 / t2452;
    let t9078 = t9077 * t79;
    let t9079 = t9078 * t747;
    let t9080 = t741 * t9079;
    let t9082 = t2586 * t2590;
    (t9070, t9072, t9073, t9078, t9079, t9080, t9082)
}
