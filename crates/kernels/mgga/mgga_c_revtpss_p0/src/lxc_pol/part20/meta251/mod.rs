//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta251 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1084;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta251<F: Float>(t11273: F, t3160: F, t2862: F, t3128: F, t1042: F, t2853: F, t3181: F, t999: F, t2866: F, t914: F, t936: F, t2869: F, t2919: F) -> (F, F, F, F, F, F, F, F) {
        let (t11277, t11280, t11281, t11285, t11286, t11289, t11291, t11293) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1084::<F>(t11273, t3160, t2862, t3128, t1042, t2853, t3181, t999, t2866, t914, t936, t2869, t2919);
    (t11277, t11280, t11281, t11285, t11286, t11289, t11291, t11293)
}
