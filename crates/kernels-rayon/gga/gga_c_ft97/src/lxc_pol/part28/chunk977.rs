//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 977/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk977(t138769: f64, t23832: f64, t2001: f64, t32773: f64, t32797: f64, t94552: f64, t136891: f64, t5824: f64, t136825: f64, t32767: f64, t32768: f64, t23839: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t138770 = t23832 * t138769;
    let t138773 = t2001 * t32773;
    let t138784 = t32797 * t94552;
    let t138794 = 0.20139801475612389137e-1_f64 * t5824 * t136891;
    let t138799 = t32767 * t136825 * t32768;
    let t138825 = t23839 * t138769;
    (t138770, t138773, t138784, t138794, t138799, t138825)
}
