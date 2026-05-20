//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta284 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1062;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1063;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1064;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta284<F: Float>(t19462: F, t225: F, t3011: F, t6205: F, t3153: F, t6305: F, t1647: F, t4980: F, t359: F, t6343: F, t1086: F, t6235: F, t4995: F, t6299: F, t1678: F, t3298: F, t342: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t19463 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1062::<F>(t19462, t225);
        let (t19467, t19501) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1063::<F>(t3011, t6205, t3153, t6305);
        let (t19526, t19556, t19566, t19569, t19572, t19602, t19603) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1064::<F>(t1647, t4980, t359, t6343, t1086, t6235, t4995, t3153, t6299, t1678, t3298, t342);
    (t19463, t19467, t19501, t19526, t19556, t19566, t19569, t19572, t19602, t19603)
}
