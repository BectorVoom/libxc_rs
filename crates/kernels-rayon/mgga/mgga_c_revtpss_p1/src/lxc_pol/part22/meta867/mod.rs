//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta867 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3023;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3024;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta867(t14325: f64, t14370: f64, t14322: f64, t2626: f64, t4398: f64, t9425: f64, t10555: f64, t14613: f64, t10565: f64, t1532: f64, t9419: f64, t162: f64, t40188: f64, t14362: f64, t9572: f64, t37: f64, t4391: f64, t14767: f64, t221: f64, t10703: f64, t2674: f64, t2661: f64, t2662: f64, t2754: f64, t4352: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50880, t50883, t50888, t50890, t50892, t50893, t50895) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3023(t14325, t14370, t14322, t2626, t4398, t9425, t10555, t14613, t10565, t1532, t9419, t162, t40188);
        let (t50901, t50903, t50933, t50937) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3024(t14362, t9572, t37, t4391, t14767, t221, t10703, t2674, t2661, t2662, t2754, t4352);
    (t50880, t50883, t50888, t50890, t50892, t50893, t50895, t50901, t50903, t50933, t50937)
}
