//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 835/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk835(t179: f64, t568: f64, t8914: f64, t1733: f64, t2645: f64, t5244: f64, t590: f64, t612: f64, t8823: f64, t8827: f64, t8832: f64, t8835: f64, t8837: f64, t8891: f64, t8894: f64, t8897: f64, t8901: f64, t8906: f64, t8911: f64) -> (f64, f64) {
    let t8916 = t179 * t8914 * t568;
    let t8919 = -0.25724410870841842183e-1_f64 * t612 * t8823 + 0.85748036236139473944e-2_f64 * t612 * t8827 + 0.42874018118069736972e-2_f64 * t612 * t8832 + 0.10003937560882938627e-2_f64 * t8835 - 0.20007875121765877254e-2_f64 * t8837 - 0.21437009059034868486e-3_f64 * t590 * t8891 + 0.10003937560882938627e-2_f64 * t8894 + 0.17149607247227894789e-2_f64 * t1733 * t8897 + 0.17149607247227894789e-2_f64 * t1733 * t8901 + 0.85748036236139473944e-3_f64 * t1733 * t8906 - 0.21437009059034868486e-3_f64 * t2645 * t8911 - 0.17149607247227894789e-2_f64 * t5244 * t8916;
    (t8916, t8919)
}
