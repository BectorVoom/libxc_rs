//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 709/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk709<F: Float>(t10442: F, t1248: F, t1720: F, t4648: F, t4889: F, t163: F, t1774: F, t4640: F, t24: F, t5005: F, t10464: F, t10450: F) -> (F, F, F, F, F) {
    let t10994 = t1248 * t1720 * t10442;
    let t10997 = t1248 * t4889 * t4648;
    let t10999 = t163 * t1774;
    let t11001 = t1248 * t10999 * t4640;
    let t11003 = t24 * t5005;
    let t11005 = t1248 * t11003 * t10464;
    let t11008 = t1248 * t1720 * t10450;
    (t10994, t10997, t11001, t11005, t11008)
}
