//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3729/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3729(t21177: f64, t3678: f64, t17303: f64, t5327: f64, t1250: f64, t12809: f64, t13099: f64, t16715: f64, t16738: f64, t16742: f64, t17212: f64, t17353: f64, t17426: f64, t17693: f64, t17732: f64, t17737: f64, t17742: f64, t17781: f64, t17784: f64, t1794: f64, t20795: f64, t20800: f64, t20802: f64, t20929: f64, t21017: f64, t3626: f64, t372: f64, t3720: f64, t44561: f64, t5331: f64, t57265: f64, t57534: f64, t70647: f64) -> f64 {
    let t70756 = t21177 * t3678;
    let t70758 = t5327 * t17303;
    let t70789 = -0.1270341277572436651e-2_f64 * t17693 * t372 * t13099 * t1794 * t1250 * t16715 - 0.96545937095505185476e-2_f64 * t70756 + 0.95275595817932748827e-4_f64 * t70758 - 0.11433071498151929859e-2_f64 * t17693 * t17353 * t1250 * t16738 - 0.57165357490759649296e-3_f64 * t17693 * t17353 * t1250 * t16742 + 0.85748036236139473944e-3_f64 * t17426 * t20802 + 0.11433071498151929859e-2_f64 * t57534 + 0.17149607247227894789e-2_f64 * t57265 * t3626 * t17737 * t17212 + 0.21437009059034868486e-3_f64 * t12809 * t3720 * t20795 * t17742 + 0.45732285992607719436e-2_f64 * t21017 * t17781 - 0.60976381323476959248e-2_f64 * t70647 * t17732 + 0.57165357490759649296e-3_f64 * t44561 * t20929 - 0.21437009059034868486e-3_f64 * t5331 * t3720 * t20800 * t17784;
    t70789
}
