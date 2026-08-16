//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta277 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1142;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1143;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta277<F: Float>(t532: F, t8107: F, t1450: F, t2107: F, t5542: F, t118: F, t1502: F, t1519: F, t1843: F, t1911: F, t2014: F, t2052: F, t2056: F, t2089: F, t2093: F, t2108: F, t4248: F, t508: F, t569: F, t651: F, t7359: F, t7732: F, t7898: F, t7969: F, t7978: F, t7984: F, t7988: F, t8065: F, t8075: F, t8079: F, t3: F, t1518: F, t7553: F, t117: F, t7983: F, t1916: F, t1918: F, t2113: F, t2115: F, t572: F, t573: F, param_d: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t8108, t8109, t8111, t8113) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1142::<F>(t532, t8107, t1450, t2107, t5542, t118, t1502, t1519, t1843, t1911, t2014, t2052, t2056, t2089, t2093, t2108, t4248, t508, t569, t651, t7359, t7732, t7898, t7969, t7978, t7984, t7988, t8065, t8075, t8079);
        let (t8114, t8118, t8124, t8127, t8130) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1143::<F>(t3, t8113, t1518, t7553, t117, t7983, t1916, t1918, t2113, t2115, t572, t573, param_d);
    (t8108, t8109, t8111, t8113, t8114, t8118, t8124, t8127, t8130)
}
