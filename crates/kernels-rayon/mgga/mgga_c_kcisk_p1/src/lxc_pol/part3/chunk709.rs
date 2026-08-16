//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 709/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk709(t10442: f64, t1248: f64, t1720: f64, t4648: f64, t4889: f64, t163: f64, t1774: f64, t4640: f64, t24: f64, t5005: f64, t10464: f64, t10450: f64) -> (f64, f64, f64, f64, f64) {
    let t10994 = t1248 * t1720 * t10442;
    let t10997 = t1248 * t4889 * t4648;
    let t10999 = t163 * t1774;
    let t11001 = t1248 * t10999 * t4640;
    let t11003 = t24 * t5005;
    let t11005 = t1248 * t11003 * t10464;
    let t11008 = t1248 * t1720 * t10450;
    (t10994, t10997, t11001, t11005, t11008)
}
