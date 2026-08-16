//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta543 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1928;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1929;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1930;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta543(t25207: f64, t29598: f64, t1468: f64, t1544: f64, t30: f64, t5962: f64, t1579: f64, t7759: f64, t7071: f64, t25262: f64, t6024: f64, t25270: f64, t6037: f64, t5980: f64, t7038: f64, t25237: f64, t5989: f64, t5993: f64, t7045: f64, t5985: f64, t7025: f64, t6019: f64, t6030: f64, t25254: f64, t25276: f64, t25284: f64, t27228: f64, t27230: f64, t28337: f64, t25220: f64, t25232: f64, t25243: f64, t28330: f64, t28333: f64, t28335: f64, t28336: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t29599, t29602, t29606, t29610, t29611, t29616, t29618) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1928(t25207, t29598, t1468, t1544, t30, t5962, t1579, t7759, t7071, t25262, t6024, t25270, t6037);
        let (t29620, t29635) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1929(t5980, t7038, t25237, t5989, t5993, t7045, t5985, t7025, t6019, t6030, t25254, t25276, t25284, t27228, t27230, t28337);
        let t29636 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1930(t25220, t25232, t25243, t28330, t28333, t28335, t28336, t29616, t29618, t29620, t29635);
    (t29599, t29602, t29606, t29610, t29611, t29636)
}
