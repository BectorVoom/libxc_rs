//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1076/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1076(t14827: f64, t865: f64, t8710: f64, t4924: f64, t903: f64, t1449: f64, t3882: f64, t11294: f64, t11356: f64, t11362: f64, t14804: f64, t14807: f64, t14810: f64, t14813: f64, t14817: f64, t14820: f64, t14824: f64, t2550: f64, t2575: f64, t2594: f64, t2619: f64, t3849: f64, t3865: f64, t3887: f64, t8847: f64, t8888: f64) -> (f64, f64) {
    let t14828 = t14827 * t865;
    let t14830 = 0.51726012919273400301e3_f64 * t8710 * t14828;
    let t14835 = t4924 * t903;
    let t14838 = t1449 * t3882;
    let t14841 = 0.64327917994770140268e2_f64 * t11294 * t3849 + 6.0_f64 * t2575 * t14804 - 4.0_f64 * t2550 * t14807 - 0.19298375398431042081e3_f64 * t8847 * t14810 - 2.0_f64 * t2550 * t14813 + 0.32163958997385070134e2_f64 * t2575 * t14817 + 0.64327917994770140268e2_f64 * t2575 * t14820 + 0.2069040516770936012e4_f64 * t8888 * t14824 - t14830 - 0.23392894490538584828e1_f64 * t11362 * t3865 + 0.34631718211362927517e2_f64 * t11356 * t3887 + 0.35089341735807877242e1_f64 * t2619 * t14835 - 0.23392894490538584828e1_f64 * t2594 * t14838;
    (t14830, t14841)
}
