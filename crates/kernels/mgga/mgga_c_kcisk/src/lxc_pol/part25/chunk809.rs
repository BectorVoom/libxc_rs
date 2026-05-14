//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 809/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk809<F: Float>(t10: F, t4594: F, t1705: F, t4850: F, t1701: F, t4908: F, t4907: F, t617: F, t608: F, t1248: F, t4644: F, t4889: F, t4648: F, t163: F, t1774: F, t4640: F) -> (F, F, F, F, F, F, F, F) {
    let t10949 = t10 * t4594;
    let t10972 = t4850 * t1705;
    let t10978 = t1701 * t4908;
    let t10982 = 1.0 / t4907 / t617;
    let t10983 = t608 * t10982;
    let t10988 = t1248 * t4889 * t4644;
    let t10997 = t1248 * t4889 * t4648;
    let t10999 = t163 * t1774;
    let t11001 = t1248 * t10999 * t4640;
    (t10949, t10972, t10978, t10983, t10988, t10997, t10999, t11001)
}
