//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta310 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1356;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1357;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1358;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta310<F: Float>(t1041: F, t10489: F, t3103: F, t3109: F, t3114: F, t376: F, t676: F, t1023: F, t248: F, t1020: F, t1017: F, t3087: F, t1015: F, t1012: F, t2928: F, t320: F, t10294: F, t268: F, t271: F, t6546: F, t2394: F, t885: F, t2772: F, t690: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10490, t10496, t10504, t10508, t10511, t10515) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1356::<F>(t1041, t10489, t3103, t3109, t3114, t376, t676, t1023, t248, t1020, t1017, t3087);
        let (t10517, t10523, t10542, t10544, t10545, t10556) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1357::<F>(t1015, t10515, t1012, t2928, t320, t10294, t268, t271, t6546, t2394, t885);
        let t10558 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1358::<F>(t2772, t690);
    (t10490, t10496, t10504, t10508, t10511, t10517, t10523, t10542, t10544, t10545, t10556, t10558)
}
