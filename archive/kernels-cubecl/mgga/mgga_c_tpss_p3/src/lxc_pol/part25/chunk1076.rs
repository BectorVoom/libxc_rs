//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1076/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1076<F: Float>(t14827: F, t865: F, t8710: F, t4924: F, t903: F, t1449: F, t3882: F, t11294: F, t11356: F, t11362: F, t14804: F, t14807: F, t14810: F, t14813: F, t14817: F, t14820: F, t14824: F, t2550: F, t2575: F, t2594: F, t2619: F, t3849: F, t3865: F, t3887: F, t8847: F, t8888: F) -> (F, F) {
    let t14828 = t14827 * t865;
    let t14830 = F::cast_from(0.51726012919273400301e3_f64) * t8710 * t14828;
    let t14835 = t4924 * t903;
    let t14838 = t1449 * t3882;
    let t14841 = F::cast_from(0.64327917994770140268e2_f64) * t11294 * t3849 + F::cast_from(6.0_f64) * t2575 * t14804 - F::cast_from(4.0_f64) * t2550 * t14807 - F::cast_from(0.19298375398431042081e3_f64) * t8847 * t14810 - F::cast_from(2.0_f64) * t2550 * t14813 + F::cast_from(0.32163958997385070134e2_f64) * t2575 * t14817 + F::cast_from(0.64327917994770140268e2_f64) * t2575 * t14820 + F::cast_from(0.2069040516770936012e4_f64) * t8888 * t14824 - t14830 - F::cast_from(0.23392894490538584828e1_f64) * t11362 * t3865 + F::cast_from(0.34631718211362927517e2_f64) * t11356 * t3887 + F::cast_from(0.35089341735807877242e1_f64) * t2619 * t14835 - F::cast_from(0.23392894490538584828e1_f64) * t2594 * t14838;
    (t14830, t14841)
}
