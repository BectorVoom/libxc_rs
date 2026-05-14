//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 658/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk658<F: Float>(t1701: F, t4908: F, t4907: F, t617: F, t608: F, t10926: F, t4911: F, t1248: F, t4644: F, t4889: F, t10488: F, t4893: F, t10442: F, t1720: F, t4648: F, t163: F, t1774: F) -> (F, F, F, F, F, F, F, F) {
    let t10978 = t1701 * t4908;
    let t10982 = 1.0 / t4907 / t617;
    let t10983 = t608 * t10982;
    let t10984 = t10926 * t4911;
    let t10988 = t1248 * t4889 * t4644;
    let t10991 = t1248 * t4893 * t10488;
    let t10994 = t1248 * t1720 * t10442;
    let t10997 = t1248 * t4889 * t4648;
    let t10999 = t163 * t1774;
    (t10978, t10983, t10984, t10988, t10991, t10994, t10997, t10999)
}
