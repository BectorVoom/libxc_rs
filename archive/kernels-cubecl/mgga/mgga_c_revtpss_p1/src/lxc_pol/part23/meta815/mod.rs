//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta815 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2660;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2661;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta815<F: Float>(t20112: F, t994: F, t4746: F, t4930: F, t19855: F, t993: F, t378: F, t15654: F, t1678: F, t225: F, t11249: F, t6299: F, t1647: F, t16565: F, t12166: F, t342: F, t12077: F, t20050: F, t3106: F, t1063: F, t247: F, t42447: F, t6092: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t64737, t64764, t64817, t64845, t64907, t65144) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2660::<F>(t20112, t994, t4746, t4930, t19855, t993, t378, t15654, t1678, t225, t11249, t6299);
        let (t65181, t65216, t65220, t65288, t65292) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2661::<F>(t1647, t16565, t12166, t1678, t342, t12077, t20050, t3106, t1063, t247, t42447, t6092);
    (t64737, t64764, t64817, t64845, t64907, t65144, t65181, t65216, t65220, t65288, t65292)
}
