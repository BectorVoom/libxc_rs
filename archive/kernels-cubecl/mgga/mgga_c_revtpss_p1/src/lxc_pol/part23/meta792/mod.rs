//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta792 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2609;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2610;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta792<F: Float>(t10777: F, t40725: F, t5988: F, t837: F, t40593: F, t6037: F, t125: F, t18392: F, t124: F, t6016: F, t14686: F, t14931: F, t4366: F, t18498: F, t221: F, t10703: F, t2674: F, t836: F, t10811: F, t18482: F) -> (F, F, F, F, F, F, F, F) {
        let (t61697, t61699, t61701, t61715) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2609::<F>(t10777, t40725, t5988, t837, t40593, t6037, t125, t18392, t124, t6016);
        let (t61718, t61727, t61749, t61754) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2610::<F>(t14686, t14931, t4366, t61715, t18498, t221, t10703, t2674, t6016, t836, t10811, t18482);
    (t61697, t61699, t61701, t61715, t61718, t61727, t61749, t61754)
}
