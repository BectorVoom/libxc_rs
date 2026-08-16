//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta885 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2798;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2799;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta885<F: Float>(t22352: F, t2435: F, t2782: F, t4086: F, t543: F, t74965: F, t4003: F, t5744: F, t74982: F, t74700: F, t4100: F, t22394: F, t686: F, t72: F, t9680: F, t21969: F, t566: F, t1450: F, t22461: F, t116: F, t21813: F, t21830: F, t625: F, t2289: F, t5916: F, t21877: F, t1507: F, t2357: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t75274, t75298, t75302, t75307, t75336) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2798::<F>(t22352, t2435, t2782, t4086, t543, t74965, t4003, t5744, t74982, t74700, t4100, t22394, t686, t72, t9680);
        let (t75379, t75389, t75439, t75526, t75540, t75542, t75625) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2799::<F>(t21969, t566, t1450, t22461, t116, t21813, t21830, t625, t2289, t5916, t21877, t1507, t2357);
    (t75274, t75298, t75302, t75307, t75336, t75379, t75389, t75439, t75526, t75540, t75542, t75625)
}
