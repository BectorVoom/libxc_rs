//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta792 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2609;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2610;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta792(t10777: f64, t40725: f64, t5988: f64, t837: f64, t40593: f64, t6037: f64, t125: f64, t18392: f64, t124: f64, t6016: f64, t14686: f64, t14931: f64, t4366: f64, t18498: f64, t221: f64, t10703: f64, t2674: f64, t836: f64, t10811: f64, t18482: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61697, t61699, t61701, t61715) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2609(t10777, t40725, t5988, t837, t40593, t6037, t125, t18392, t124, t6016);
        let (t61718, t61727, t61749, t61754) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2610(t14686, t14931, t4366, t61715, t18498, t221, t10703, t2674, t6016, t836, t10811, t18482);
    (t61697, t61699, t61701, t61715, t61718, t61727, t61749, t61754)
}
