//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1648;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1649;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta403<F: Float>(t15700: F, t15702: F, t3578: F, t1215: F, t607: F, t475: F, t4728: F, t1735: F, t3243: F, t11668: F, t1744: F, t3540: F, t1731: F, t1222: F, t4961: F, t1743: F, t3566: F, t11692: F, t1174: F, t11834: F, t15686: F, t15691: F, t15699: F, t3552: F, t3557: F, t3562: F, t3577: F, t488: F, t4889: F) -> (F, F, F, F, F, F) {
        let (t15704, t15707, t15708, t15710, t15714, t15717) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1648::<F>(t15700, t15702, t3578, t1215, t607, t475, t4728, t1735, t3243, t11668, t1744, t3540);
        let t15726 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1649::<F>(t1731, t3540, t1222, t4961, t1743, t3566, t11692, t1174, t11834, t15686, t15691, t15699, t15704, t15710, t15714, t15717, t3552, t3557, t3562, t3577, t488, t4889);
    (t15704, t15707, t15708, t15710, t15714, t15726)
}
