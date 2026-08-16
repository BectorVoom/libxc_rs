//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta526 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1954;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1955;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta526(t14224: f64, t25931: f64, t72: f64, t7920: f64, t686: f64, t25895: f64, t25878: f64, t25882: f64, t25893: f64, t25896: f64, t25921: f64, t25930: f64, t27837: f64, t27841: f64, t27846: f64, t27853: f64, t27858: f64, t27861: f64, t27865: f64, t27868: f64, t7295: f64, t7304: f64, t7926: f64, t1426: f64, t27836: f64, t7063: f64, t7286: f64, t7929: f64, t7284: f64, t7289: f64, t1444: f64, t7296: f64, t7910: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27869, t27872, t27873, t27879) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1954(t14224, t25931, t72, t7920, t686, t25895, t25878, t25882, t25893, t25896, t25921, t25930, t27837, t27841, t27846, t27853, t27858, t27861, t27865, t27868, t7295, t7304, t7926);
        let (t27883, t27884, t27885, t27887, t27888, t27889, t27891, t27896) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1955(t1426, t27836, t7063, t7286, t72, t7929, t686, t7284, t7289, t1444, t7296, t7910);
    (t27869, t27872, t27873, t27879, t27883, t27884, t27885, t27887, t27888, t27889, t27891, t27896)
}
