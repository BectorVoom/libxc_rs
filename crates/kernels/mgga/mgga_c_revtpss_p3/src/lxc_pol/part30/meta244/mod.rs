//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta244 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1087;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1088;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta244<F: Float>(t5351: F, t5458: F, t3766: F, t487: F, t460: F, t3302: F, t3603: F, t1248: F, t5332: F, t1269: F, t1287: F, t1794: F, t5284: F, t3781: F, t471: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5459, t5462, t5463, t5464) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1087::<F>(t5351, t5458, t3766, t487, t460, t3302, t3603);
        let t5465 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1088::<F>(t1248, t5464);
        let (t5466, t5470, t5474, t5477, t5478, t5479, t5480) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1089::<F>(t5332, t5465, t1269, t1287, t1794, t487, t5284, t3781, t460, t1248, t3302, t471);
    (t5459, t5462, t5463, t5464, t5465, t5466, t5470, t5474, t5477, t5478, t5479, t5480)
}
