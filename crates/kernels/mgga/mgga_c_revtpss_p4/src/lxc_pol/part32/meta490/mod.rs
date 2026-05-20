//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1745;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1746;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta490<F: Float>(t28384: F, t7076: F, t1580: F, t7384: F, t689: F, t213: F, t7997: F, t25383: F, t26498: F, t26500: F, t26547: F, t28361: F, t28366: F, t28369: F, t28371: F, t28374: F, t28378: F, t7067: F, t7070: F, t8012: F, t8016: F, t887: F, t233: F, t28340: F, t1957: F, t2061: F, t231: F, t4423: F, t25317: F, t8006: F, t886: F, t4533: F, t7071: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t28385, t28390, t28391, t28394, t28397) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1745::<F>(t28384, t7076, t1580, t7384, t689, t213, t7997, t25383, t26498, t26500, t26547, t28361, t28366, t28369, t28371, t28374, t28378, t7067, t7070, t8012, t8016, t887);
        let (t28399, t28400, t28404, t28405, t28411, t28417, t28418) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1746::<F>(t233, t28340, t1957, t2061, t231, t4423, t7076, t25317, t8006, t886, t4533, t7071);
    (t28385, t28390, t28391, t28394, t28397, t28399, t28400, t28404, t28405, t28411, t28417, t28418)
}
