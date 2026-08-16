//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta193 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1000;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1001;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1002;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1003;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1004;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta193<F: Float>(t1484: F, t2523: F, t2408: F, t2417: F, t2423: F, t2426: F, t2486: F, t2518: F, t2522: F, t2530: F, t2537: F, t2538: F, t2665: F, t4209: F, t4213: F, t4214: F, t4215: F, t4216: F, t4319: F, t2: F, t265: F, t584: F, t1540: F, t690: F, t1409: F, t2770: F, t607: F, t2768: F, t123: F, t2775: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t4323 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1000::<F>(t1484, t2523, t2408, t2417, t2423, t2426, t2486, t2518, t2522, t2530, t2537, t2538, t2665, t4209, t4213, t4214, t4215, t4216);
        let t4324 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1001::<F>(t4319, t4323);
        let (t4331, t4332, t4335) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1002::<F>(t2, t265, t584, t1540, t690);
        let (t4337, t4338) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1003::<F>(t1409, t2770, t607);
        let (t4339, t4340, t4342, t4343) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1004::<F>(t2768, t4338, t123, t1409, t2775, t607);
    (t4324, t4331, t4332, t4335, t4337, t4338, t4339, t4340, t4342, t4343)
}
