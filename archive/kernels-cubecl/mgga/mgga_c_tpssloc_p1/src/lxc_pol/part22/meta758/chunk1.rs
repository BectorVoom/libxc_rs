//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2545/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2545<F: Float>(t1102: F, t5999: F, t14801: F, t14804: F, t45192: F, t48140: F, t68513: F, t50822: F, t44938: F, t43777: F, t43859: F, t43895: F, t50919: F, t50948: F, t71203: F, t71206: F) -> (F, F, F, F, F, F) {
    let t71498 = t5999 * t1102;
    let t71499 = t14801 * t71498;
    let t71501 = t14804 * t71498;
    let t71505 = t48140 * t45192 * t68513;
    let t71508 = t48140 * t50822 * t68513;
    let t71511 = t48140 * t44938 * t68513;
    let t71515 = F::cast_from(0.181155e1_f64) * t71203 + F::cast_from(0.543465e1_f64) * t71206 + t43777 + F::cast_from(0.58258125e1_f64) * t71499 - F::cast_from(0.1237865625e0_f64) * t71501 - F::cast_from(0.24528888888888888889e0_f64) * t43859 - F::cast_from(0.49671e0_f64) * t71505 + F::cast_from(0.149013e1_f64) * t71508 + F::cast_from(0.11038e0_f64) * t71511 - F::cast_from(0.26837777777777777779e0_f64) * t50919 + F::cast_from(0.80513333333333333336e0_f64) * t50948 + t43895;
    (t71499, t71501, t71505, t71508, t71511, t71515)
}
