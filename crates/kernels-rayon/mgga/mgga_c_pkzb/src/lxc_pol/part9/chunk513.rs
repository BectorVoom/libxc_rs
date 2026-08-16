//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 513/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk513(t2105: f64, t2107: f64, t2006: f64, t2012: f64, t2026: f64, t2033: f64, t2038: f64, t2041: f64, t2047: f64, t2051: f64, t2054: f64, t2057: f64, t2060: f64, t2067: f64, t2071: f64, t2074: f64, t2082: f64, t2085: f64, t2091: f64, t2096: f64, t2101: f64, t2104: f64, t276: f64, t279: f64, t299: f64, t303: f64, t735: f64, t744: f64, t757: f64, t763: f64, t771: f64, t782: f64) -> (f64, f64) {
    let t2108 = t2105 * t2107;
    let t2111 = 0.12862205435420921092e-2_f64 * t299 * t2006 + 0.21437009059034868486e-3_f64 * t757 * t2012 + 0.42874018118069736972e-3_f64 * t2026 * t2033 - 0.21437009059034868486e-3_f64 * t2038 * t2041 - t2047 - t2051 / 144.0_f64 - t276 * t2054 / 96.0_f64 + 11.0_f64 / 108.0_f64 * t2057 * t279 - t2060 / 54.0_f64 + t735 * t744 / 18.0_f64 - t2067 - 0.57165357490759649296e-3_f64 * t2071 - 0.42874018118069736972e-3_f64 * t299 * t2074 + 0.72409452821628889107e-2_f64 * t2082 * t303 - 0.15244095330869239812e-2_f64 * t2085 + 0.45732285992607719436e-2_f64 * t771 * t782 + t276 * t2091 / 48.0_f64 - 0.22866142996303859718e-2_f64 * t2096 * t763 + 0.28582678745379824648e-3_f64 * t2101 - 0.85748036236139473944e-3_f64 * t2104 * t2108;
    (t2108, t2111)
}
