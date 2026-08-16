//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta406 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1655;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1656;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta406<F: Float>(t15030: F, t15785: F, t1241: F, t1251: F, t5088: F, t3598: F, t1760: F, t3599: F, t11606: F, t225: F, t4941: F, t1751: F, t3481: F, t3630: F, t1238: F, t1252: F, t14972: F, t14980: F, t3487: F, t3593: F, t3600: F, t3631: F, t498: F, t5055: F, t5060: F, t5089: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15786, t15787, t15789, t15790, t15793, t15794, t15797, t15800) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1655::<F>(t15030, t15785, t1241, t1251, t5088, t3598, t1760, t3599, t11606, t225, t4941, t1751, t3481);
        let (t15802, t15803, t15806) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1656::<F>(t1760, t3630, t3598, t1238, t1252, t14972, t14980, t15787, t15790, t15794, t15797, t15800, t3487, t3593, t3600, t3631, t498, t5055, t5060, t5089);
    (t15786, t15787, t15789, t15790, t15793, t15794, t15797, t15800, t15802, t15803, t15806)
}
