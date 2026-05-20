//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta515 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1868;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1869;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta515<F: Float>(t1089: F, t1668: F, t25681: F, t4866: F, t7168: F, t7828: F, t988: F, t7160: F, t1078: F, t11239: F, t1035: F, t1983: F, t1976: F, t3153: F, t4998: F, t1043: F, t1097: F, t25591: F, t25605: F, t25640: F, t25651: F, t27595: F, t27599: F, t27606: F, t27609: F, t27616: F, t27621: F, t4758: F, t7144: F, t7156: F, t7162: F, t7167: F, t7170: F, t7174: F, t7825: F, t7833: F, t7837: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t27627, t27631, t27635, t27638, t27639, t27640) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1868::<F>(t1089, t1668, t25681, t4866, t7168, t7828, t988, t7160, t1078, t11239, t1035, t1983);
        let (t27641, t27642, t27643, t27647, t27650) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1869::<F>(t1668, t1976, t3153, t4998, t1043, t1089, t7828, t1097, t25591, t25605, t25640, t25651, t27595, t27599, t27606, t27609, t27616, t27621, t27627, t27631, t27635, t27640, t4758, t7144, t7156, t7162, t7167, t7170, t7174, t7825, t7833, t7837);
    (t27627, t27631, t27635, t27638, t27639, t27640, t27641, t27642, t27643, t27647, t27650)
}
