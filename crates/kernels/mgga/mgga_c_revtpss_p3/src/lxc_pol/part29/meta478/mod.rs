//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta478 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1754;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta478<F: Float>(t1561: F, t25266: F, t25270: F, t4462: F, t4447: F, t4452: F, t1945: F, t4371: F, t807: F, t4458: F, t7025: F, t1549: F, t25277: F) -> (F, F, F, F, F, F, F, F) {
        let (t27230, t27232, t27234, t27236, t27239, t27240, t27244, t27246) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1754::<F>(t1561, t25266, t25270, t4462, t4447, t4452, t1945, t4371, t807, t4458, t7025, t1549, t25277);
    (t27230, t27232, t27234, t27236, t27239, t27240, t27244, t27246)
}
