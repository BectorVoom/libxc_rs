//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1648;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1649;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta403(t15700: f64, t15702: f64, t3578: f64, t1215: f64, t607: f64, t475: f64, t4728: f64, t1735: f64, t3243: f64, t11668: f64, t1744: f64, t3540: f64, t1731: f64, t1222: f64, t4961: f64, t1743: f64, t3566: f64, t11692: f64, t1174: f64, t11834: f64, t15686: f64, t15691: f64, t15699: f64, t3552: f64, t3557: f64, t3562: f64, t3577: f64, t488: f64, t4889: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t15704, t15707, t15708, t15710, t15714, t15717) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1648(t15700, t15702, t3578, t1215, t607, t475, t4728, t1735, t3243, t11668, t1744, t3540);
        let t15726 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1649(t1731, t3540, t1222, t4961, t1743, t3566, t11692, t1174, t11834, t15686, t15691, t15699, t15704, t15710, t15714, t15717, t3552, t3557, t3562, t3577, t488, t4889);
    (t15704, t15707, t15708, t15710, t15714, t15726)
}
