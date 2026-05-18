//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 513/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk513<F: Float>(t2105: F, t2107: F, t2006: F, t2012: F, t2026: F, t2033: F, t2038: F, t2041: F, t2047: F, t2051: F, t2054: F, t2057: F, t2060: F, t2067: F, t2071: F, t2074: F, t2082: F, t2085: F, t2091: F, t2096: F, t2101: F, t2104: F, t276: F, t279: F, t299: F, t303: F, t735: F, t744: F, t757: F, t763: F, t771: F, t782: F) -> (F, F) {
    let t2108 = t2105 * t2107;
    let t2111 = F::new(0.12862205435420921092e-2) * t299 * t2006 + F::new(0.21437009059034868486e-3) * t757 * t2012 + F::new(0.42874018118069736972e-3) * t2026 * t2033 - F::new(0.21437009059034868486e-3) * t2038 * t2041 - t2047 - t2051 / F::new(144.0) - t276 * t2054 / F::new(96.0) + F::new(11.0) / F::new(108.0) * t2057 * t279 - t2060 / F::new(54.0) + t735 * t744 / F::new(18.0) - t2067 - F::new(0.57165357490759649296e-3) * t2071 - F::new(0.42874018118069736972e-3) * t299 * t2074 + F::new(0.72409452821628889107e-2) * t2082 * t303 - F::new(0.15244095330869239812e-2) * t2085 + F::new(0.45732285992607719436e-2) * t771 * t782 + t276 * t2091 / F::new(48.0) - F::new(0.22866142996303859718e-2) * t2096 * t763 + F::new(0.28582678745379824648e-3) * t2101 - F::new(0.85748036236139473944e-3) * t2104 * t2108;
    (t2108, t2111)
}
