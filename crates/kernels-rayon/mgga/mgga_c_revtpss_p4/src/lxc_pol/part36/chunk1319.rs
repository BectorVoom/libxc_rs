//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1319/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1319(t113440: f64, t27799: f64, t100987: f64, t29598: f64, t113103: f64, t25759: f64, t113432: f64, t1711: f64, t5962: f64, t5966: f64, t6079: f64, t23279: f64, t27763: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t114101 = t27799 * t113440;
    let t114104 = t100987 * t29598;
    let t114107 = t25759 * t113103;
    let t114110 = t25759 * t113432;
    let t114113 = t1711 * t5962;
    let t114117 = t1711 * t5966;
    let t114121 = t1711 * t6079;
    let t114128 = t27763 * t23279;
    (t114101, t114104, t114107, t114110, t114113, t114117, t114121, t114128)
}
