//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta384 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1459;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1460;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta384<F: Float>(t16673: F, t816: F, t13278: F, t1512: F, t5587: F, t9667: F, t1510: F, t4255: F, t13350: F, t120: F, t5611: F, t4180: F, t4182: F, t5527: F, t829: F, t9646: F, t5544: F, t2645: F, t16839: F, t2647: F, t13177: F, t13251: F, t13260: F, t13275: F, t13277: F, t13280: F, t13287: F, t13320: F, t13330: F, t2643: F, t4167: F, t4178: F, t4191: F, t4236: F, t4240: F, t4250: F, t831: F) -> (F, F, F, F, F, F, F, F) {
        let (t16872, t16877, t16879, t16887, t16888, t16891, t16893) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1459::<F>(t16673, t816, t13278, t1512, t5587, t9667, t1510, t4255, t13350, t120, t5611, t4180, t4182);
        let (t16898, t16903, t16907, t16910) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1460::<F>(t120, t5527, t829, t9646, t5544, t2645, t16839, t2647, t13177, t13251, t13260, t13275, t13277, t13280, t13287, t13320, t13330, t1512, t16872, t16877, t16879, t16888, t16893, t2643, t4167, t4178, t4191, t4236, t4240, t4250, t831);
    (t16887, t16888, t16891, t16893, t16898, t16903, t16907, t16910)
}
