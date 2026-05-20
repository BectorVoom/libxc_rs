//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta289 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1703;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta289<F: Float>(t1379: F, t9709: F, t2689: F, t3952: F, t1413: F, t3889: F, t547: F, t807: F, t9646: F, t2236: F, t66: F) -> (F, F, F, F, F, F, F) {
        let (t9711, t9712, t9714, t9715, t9716, t9718, t9720) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1703::<F>(t1379, t9709, t2689, t3952, t1413, t3889, t547, t807, t9646, t2236, t66);
    (t9711, t9712, t9714, t9715, t9716, t9718, t9720)
}
