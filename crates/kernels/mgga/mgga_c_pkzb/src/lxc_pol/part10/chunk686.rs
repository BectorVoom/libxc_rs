//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 686/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk686<F: Float>(t2989: F, t790: F, t1134: F, t1144: F, t2957: F, t2965: F, t307: F, t311: F, t786: F, t800: F) -> (F, F) {
    let t2990 = t790 * t2989;
    let t2993 = 0.65854491829355115987e0 * t2957 * t311 - 0.65854491829355115987e0 * t1134 * t800 - 0.65854491829355115987e0 * t786 * t1144 + 0.13170898365871023197e1 * t307 * t2965 - 0.65854491829355115987e0 * t307 * t2990;
    (t2990, t2993)
}
