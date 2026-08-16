//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta220 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1053;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1054;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1055;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta220<F: Float>(t457: F, t4928: F, t460: F, t974: F, t1184: F, t1714: F, t1174: F, t1180: F, t1187: F, t3430: F, t3433: F, t3436: F, t3447: F, t4887: F, t4889: F, t4897: F, t4901: F, t4905: F, t4909: F, t4913: F, t4917: F, t4920: F, t491: F, t1235: F, t1720: F, t1721: F, t225: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4929, t4930) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1053::<F>(t457, t4928, t460);
        let (t4934, t4935, t4936, t4940) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1054::<F>(t4930, t974, t457, t1184, t1714, t460, t1174, t1180, t1187, t3430, t3433, t3436, t3447, t4887, t4889, t4897, t4901, t4905, t4909, t4913, t4917, t4920);
        let (t4941, t4943, t4945) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1055::<F>(t491, t4940, t1235, t1720, t1721, t225);
    (t4929, t4930, t4934, t4935, t4936, t4940, t4941, t4943, t4945)
}
