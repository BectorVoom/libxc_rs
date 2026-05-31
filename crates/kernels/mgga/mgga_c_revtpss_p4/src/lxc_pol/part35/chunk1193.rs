//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1193/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1193<F: Float>(t114: F, t1450: F, t22809: F, t1907: F, t6922: F, t1868: F, t6781: F, t22633: F, t94: F, t6816: F, t101451: F, t105870: F, t105878: F, t114394: F, t114396: F, t114398: F, t95397: F) -> (F, F, F, F, F, F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t114776 = t1450 * t22809;
    let t114780 = t1907 * t6922;
    let t114791 = t1868 * t6781;
    let t114800 = t1868 * t6922;
    let t114812 = t94 * t22633;
    let t114820 = t6816 * t1907;
    let t114905 = piecewise3::<F>(t115, F::cast_from(0.0_f64), -t95397 - F::cast_from(22.0_f64) / F::cast_from(3.0_f64) * t101451 - F::cast_from(4.0_f64) * t105870 + F::cast_from(2.0_f64) * t105878 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t114394 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t114396 - t114398 / F::cast_from(4.0_f64));
    (t114776, t114780, t114791, t114800, t114812, t114820, t114905)
}
