//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta555 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2113;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta555<F: Float>(t1903: F, t5774: F, t4076: F, t6918: F, t72: F, t686: F, t3915: F, t6889: F, t786: F, t1364: F, t14100: F, t5722: F) -> (F, F, F, F, F, F, F, F) {
        let (t22394, t22395, t22398, t22399, t22400, t22404, t22405, t22407) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2113::<F>(t1903, t5774, t4076, t6918, t72, t686, t3915, t6889, t786, t1364, t14100, t5722);
    (t22394, t22395, t22398, t22399, t22400, t22404, t22405, t22407)
}
