//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1194/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1194(t1731: f64, t3540: f64, t1222: f64, t4961: f64, t1743: f64, t3566: f64, t11692: f64, t1174: f64, t11834: f64, t15686: f64, t15691: f64, t15699: f64, t15704: f64, t15710: f64, t15714: f64, t15717: f64, t3552: f64, t3557: f64, t3562: f64, t3577: f64, t488: f64, t4889: f64) -> f64 {
    let t15719 = t1731 * t3540;
    let t15722 = t4961 * t1222 / 432.0_f64;
    let t15723 = t3566 * t1743;
    let t15726 = t1174 * t15686 / 36.0_f64 - t15691 + t4889 * t3552 / 108.0_f64 + t4889 * t3557 / 54.0_f64 - t4889 * t3562 / 81.0_f64 + t15699 + t11692 * t15704 / 2304.0_f64 - t3577 * t15710 / 1152.0_f64 + t11834 + 5.0_f64 / 13824.0_f64 * t3577 * t15714 + t15717 / 2592.0_f64 - t15719 / 13824.0_f64 - t15722 - t15723 * t488 / 576.0_f64;
    t15726
}
