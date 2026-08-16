//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1745;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1746;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta490(t28384: f64, t7076: f64, t1580: f64, t7384: f64, t689: f64, t213: f64, t7997: f64, t25383: f64, t26498: f64, t26500: f64, t26547: f64, t28361: f64, t28366: f64, t28369: f64, t28371: f64, t28374: f64, t28378: f64, t7067: f64, t7070: f64, t8012: f64, t8016: f64, t887: f64, t233: f64, t28340: f64, t1957: f64, t2061: f64, t231: f64, t4423: f64, t25317: f64, t8006: f64, t886: f64, t4533: f64, t7071: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28385, t28390, t28391, t28394, t28397) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1745(t28384, t7076, t1580, t7384, t689, t213, t7997, t25383, t26498, t26500, t26547, t28361, t28366, t28369, t28371, t28374, t28378, t7067, t7070, t8012, t8016, t887);
        let (t28399, t28400, t28404, t28405, t28411, t28417, t28418) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1746(t233, t28340, t1957, t2061, t231, t4423, t7076, t25317, t8006, t886, t4533, t7071);
    (t28385, t28390, t28391, t28394, t28397, t28399, t28400, t28404, t28405, t28411, t28417, t28418)
}
