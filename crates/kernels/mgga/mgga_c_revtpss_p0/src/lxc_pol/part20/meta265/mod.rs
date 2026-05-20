//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta265 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1112;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta265<F: Float>(t11249: F, t11631: F, t11248: F, t1042: F, t2251: F, t999: F, t4801: F, t1041: F, t1047: F, t1063: F, t11233: F, t11246: F, t11252: F, t11256: F, t11259: F, t11264: F, t11268: F, t11271: F, t11274: F, t11277: F, t11281: F, t11286: F, t11623: F, t11630: F, t3124: F, t3127: F, t3136: F, t3157: F, t3164: F) -> (F, F, F, F, F, F) {
        let (t11633, t11634, t11637, t11638, t11639, t11642) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1112::<F>(t11249, t11631, t11248, t1042, t2251, t999, t4801, t1041, t1047, t1063, t11233, t11246, t11252, t11256, t11259, t11264, t11268, t11271, t11274, t11277, t11281, t11286, t11623, t11630, t3124, t3127, t3136, t3157, t3164);
    (t11633, t11634, t11637, t11638, t11639, t11642)
}
