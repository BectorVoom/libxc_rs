//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1805;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1806;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta421<F: Float>(t5941: F, t72: F, t757: F, t10569: F, t4186: F, t4402: F, t4401: F, t177: F, t762: F, t10579: F, t14386: F, t1522: F, t10566: F, t10568: F, t10577: F, t10582: F, t10584: F, t10586: F, t9514: F, t9517: F, t9521: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18555, t18556, t18557, t18558, t18559, t18561, t18562, t18563, t18564, t18565, t18567) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1805::<F>(t5941, t72, t757, t10569, t4186, t4402, t4401, t177, t762, t10579, t14386, t1522);
        let t18568 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1806::<F>(t10566, t10568, t10577, t10582, t10584, t10586, t18557, t18558, t18561, t18564, t18565, t18567, t9514, t9517, t9521);
    (t18555, t18556, t18557, t18558, t18559, t18561, t18562, t18563, t18564, t18565, t18567, t18568)
}
