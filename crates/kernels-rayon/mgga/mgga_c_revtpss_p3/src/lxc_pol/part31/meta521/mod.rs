//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta521 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1883;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1884;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1885;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta521(t14224: f64, t25931: f64, t72: f64, t7920: f64, t686: f64, t25895: f64, t25878: f64, t25882: f64, t25893: f64, t25896: f64, t25921: f64, t25930: f64, t27837: f64, t27841: f64, t27846: f64, t27853: f64, t27858: f64, t27861: f64, t27865: f64, t27868: f64, t7295: f64, t7304: f64, t7926: f64, t1426: f64, t27836: f64, t7063: f64, t7286: f64, t7929: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27869, t27872, t27873) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1883(t14224, t25931, t72, t7920, t686);
        let (t27874, t27876, t27879) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1884(t25895, t27873, t25878, t25882, t25893, t25896, t25921, t25930, t27837, t27841, t27846, t27853, t27858, t27861, t27865, t27868, t27869, t7295, t7304, t7926);
        let t27883 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1885(t1426, t27836);
        let (t27884, t27885, t27887, t27888) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1886(t27883, t7063, t7286, t72, t7929, t686);
    (t27869, t27872, t27873, t27874, t27876, t27879, t27883, t27884, t27885, t27887, t27888)
}
