//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1421;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta368<F: Float>(t11153: F, t3584: F, t1734: F, t3508: F, t3548: F, t4889: F, t135: F, t5045: F, t1174: F, t1222: F, t4966: F, t1215: F) -> (F, F, F, F, F, F, F) {
        let (t15654, t15659, t15671, t15689, t15691, t15699, t15700) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1421::<F>(t11153, t3584, t1734, t3508, t3548, t4889, t135, t5045, t1174, t1222, t4966, t1215);
    (t15654, t15659, t15671, t15689, t15691, t15699, t15700)
}
