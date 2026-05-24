//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 491/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk491<F: Float>(t397: F, t4889: F, t662: F, t656: F, t1774: F, t25: F) -> (F, F, F) {
    let t4995 = t397 * t4889 * t662;
    let t4997 = F::cast_from(0.11993859144118211475e-1_f64) * t656 * t4995;
    let t4998 = t25 * t1774;
    (t4995, t4997, t4998)
}
