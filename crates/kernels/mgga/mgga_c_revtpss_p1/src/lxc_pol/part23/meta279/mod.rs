//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta279 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1499;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1500;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1501;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1502;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta279<F: Float>(t136: F, t853: F, t220: F, t2723: F, t775: F, t820: F, t823: F, t844: F, t2681: F, t839: F, t222: F, t9727: F, t2737: F, t9802: F, t2482: F, t596: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t10778, t10779) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1499::<F>(t136, t853, t220);
        let (t10786, t10811) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1500::<F>(t2723, t775, t820, t823, t844);
        let t10815 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1501::<F>(t2681, t820, t823);
        let (t10816, t10824, t10826, t10845) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1502::<F>(t10815, t839, t222, t9727, t2737, t9802, t2482, t596, t823);
    (t10778, t10779, t10786, t10811, t10815, t10816, t10824, t10826, t10845)
}
