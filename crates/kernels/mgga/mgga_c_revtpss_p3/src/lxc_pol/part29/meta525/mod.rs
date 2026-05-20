//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta525 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1852;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1853;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta525<F: Float>(t7282: F, t93139: F, t1955: F, t25920: F, t4075: F, t2028: F, t3999: F, t25875: F, t4004: F, t676: F, t25894: F, t25877: F, t94382: F, t25304: F, t25949: F, t1419: F, t7063: F, t25898: F, t9656: F, t281: F, t555: F, t93238: F, t1426: F, t94609: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94701, t94705, t94763, t94764, t94768, t94771) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1852::<F>(t7282, t93139, t1955, t25920, t4075, t2028, t3999, t25875, t4004, t676, t25894, t25877, t94382);
        let (t94776, t94801, t94802, t94823, t94849, t94878) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1853::<F>(t25304, t25949, t1419, t7063, t25898, t1955, t7282, t9656, t281, t555, t93238, t1426, t94609);
    (t94701, t94705, t94763, t94764, t94768, t94771, t94776, t94801, t94802, t94823, t94849, t94878)
}
