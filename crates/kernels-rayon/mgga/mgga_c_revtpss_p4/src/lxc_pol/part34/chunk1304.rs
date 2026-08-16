//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1304/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1304(t113103: f64, t25759: f64, t113432: f64, t1711: f64, t5962: f64, t5966: f64, t6079: f64, t23279: f64, t27763: f64, t6075: f64, t106516: f64, t114101: f64, t114104: f64, t1940: f64, t1963: f64, t2403: f64, t25206: f64, t25445: f64, t27158: f64, t27368: f64, t29705: f64, t29946: f64, t29949: f64, t29967: f64, t4541: f64, t6416: f64, t7091: f64, t7783: f64, t7869: f64, t98637: f64) -> f64 {
    let t114107 = t25759 * t113103;
    let t114110 = t25759 * t113432;
    let t114113 = t1711 * t5962;
    let t114117 = t1711 * t5966;
    let t114121 = t1711 * t6079;
    let t114128 = t27763 * t23279;
    let t114140 = t1711 * t6075;
    let t114149 = 9.0_f64 * t25206 * t114101 - 9.0_f64 * t25206 * t114104 - 9.0_f64 / 2.0_f64 * t25206 * t114107 - 9.0_f64 / 2.0_f64 * t25206 * t114110 + 9.0_f64 / 2.0_f64 * t2403 * t1963 * t114113 + 9.0_f64 * t4541 * t1963 * t114117 + 3.0_f64 * t1940 * t25445 * t114121 - 3.0_f64 * t1940 * t27368 * t29967 + 9.0_f64 * t27158 * t114128 + 3.0_f64 / 2.0_f64 * t1940 * t29705 * t1711 + 3.0_f64 / 2.0_f64 * t1940 * t7783 * t6416 - 3.0_f64 / 2.0_f64 * t1940 * t106516 * t7869 - 3.0_f64 / 2.0_f64 * t1940 * t7091 * t114140 - 9.0_f64 * t98637 * t29946 + 9.0_f64 * t2403 * t7783 * t29949;
    t114149
}
