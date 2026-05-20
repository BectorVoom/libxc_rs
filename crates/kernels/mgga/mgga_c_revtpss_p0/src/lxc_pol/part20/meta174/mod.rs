//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta174 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk913;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta174<F: Float>(t1331: F, t3857: F, t189: F, t9363: F, t512: F, t3850: F, t72: F, t757: F, t2619: F, t3825: F, t1333: F, t3863: F) -> (F, F, F, F, F, F, F, F) {
        let (t9560, t9561, t9562, t9563, t9565, t9567, t9569, t9570) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk913::<F>(t1331, t3857, t189, t9363, t512, t3850, t72, t757, t2619, t3825, t1333, t3863);
    (t9560, t9561, t9562, t9563, t9565, t9567, t9569, t9570)
}
