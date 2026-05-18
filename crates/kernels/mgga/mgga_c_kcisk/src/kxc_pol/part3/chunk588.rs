//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 588/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk588<F: Float>(t397: F, t4889: F, t662: F, t656: F, t1774: F, t25: F) -> (F, F, F) {
    let t4995 = t397 * t4889 * t662;
    let t4997 = F::new(0.11993859144118211475e-1) * t656 * t4995;
    let t4998 = t25 * t1774;
    (t4995, t4997, t4998)
}
