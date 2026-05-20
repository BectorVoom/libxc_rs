//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta543 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1928;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1929;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1930;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta543<F: Float>(t25207: F, t29598: F, t1468: F, t1544: F, t30: F, t5962: F, t1579: F, t7759: F, t7071: F, t25262: F, t6024: F, t25270: F, t6037: F, t5980: F, t7038: F, t25237: F, t5989: F, t5993: F, t7045: F, t5985: F, t7025: F, t6019: F, t6030: F, t25254: F, t25276: F, t25284: F, t27228: F, t27230: F, t28337: F, t25220: F, t25232: F, t25243: F, t28330: F, t28333: F, t28335: F, t28336: F) -> (F, F, F, F, F, F) {
        let (t29599, t29602, t29606, t29610, t29611, t29616, t29618) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1928::<F>(t25207, t29598, t1468, t1544, t30, t5962, t1579, t7759, t7071, t25262, t6024, t25270, t6037);
        let (t29620, t29635) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1929::<F>(t5980, t7038, t25237, t5989, t5993, t7045, t5985, t7025, t6019, t6030, t25254, t25276, t25284, t27228, t27230, t28337);
        let t29636 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1930::<F>(t25220, t25232, t25243, t28330, t28333, t28335, t28336, t29616, t29618, t29620, t29635);
    (t29599, t29602, t29606, t29610, t29611, t29636)
}
