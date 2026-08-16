//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta452 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1721;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1722;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta452<F: Float>(t3172: F, t5303: F, t1261: F, t17633: F, t5352: F, t3720: F, t1209: F, t489: F, t3623: F, t370: F, t1214: F, t606: F, t5051: F, t3626: F, t3566: F, t1121: F, t1774: F, t3584: F, t471: F, t5351: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17720, t17721, t17724, t17727, t17728, t17729, t17730) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1721::<F>(t3172, t5303, t1261, t17633, t5352, t3720, t1209, t489, t3623, t370, t1214, t606);
        let (t17732, t17735, t17736, t17739, t17744) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1722::<F>(t17730, t5051, t3626, t3566, t489, t17728, t1121, t1774, t3584, t471, t5351, t3720);
    (t17720, t17721, t17724, t17727, t17729, t17730, t17732, t17735, t17736, t17739, t17744)
}
