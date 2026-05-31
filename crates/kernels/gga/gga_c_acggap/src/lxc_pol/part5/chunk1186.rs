//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1186/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1186<F: Float>(t3621: F, t5916: F, t1137: F, t5919: F, t1084: F, t1090: F, t1181: F, t16674: F, t16676: F, t16678: F, t16680: F, t16686: F, t16688: F, t1879: F, t20545: F, t3396: F, t367: F, t4479: F, t4593: F, t4735: F, t5187: F) -> F {
    let t21557 = t3621 * t5916;
    let t21559 = t1137 * t5919;
    let t21575 = t367 * t4593 * t5187 / F::cast_from(12.0_f64) + t367 * t4593 * t4479 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t21557 - F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t21559 - F::cast_from(0.20579528696673473748e-1_f64) * t3396 * t1181 * t1879 * t1090 - F::cast_from(0.20579528696673473748e-1_f64) * t4735 * t1181 * t20545 * t1084 + F::cast_from(0.24009450146119052704e-1_f64) * t16674 + F::cast_from(0.45351183609335988442e-1_f64) * t16676 - F::cast_from(0.45351183609335988442e-1_f64) * t16678 - F::cast_from(0.17149607247227894789e-2_f64) * t16680 + F::cast_from(0.68026775414003982663e-1_f64) * t16686 + F::cast_from(0.45351183609335988442e-1_f64) * t16688;
    t21575
}
