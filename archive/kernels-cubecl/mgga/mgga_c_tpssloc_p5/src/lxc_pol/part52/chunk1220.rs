//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1220/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1220<F: Float>(t1983: F, t33129: F, t191: F, t192: F, t7681: F, t2020: F, t3701: F, t7752: F, t2019: F, t1873: F, t24999: F, t33085: F) -> (F, F, F, F, F, F, F, F) {
    let t33131 = F::cast_from(3.0_f64) * t1983 * t33129;
    let t33133 = t7681 * t191 * t192;
    let t33134 = t33133 * t2020;
    let t33136 = t3701 * t7752;
    let t33137 = t2019 * t33136;
    let t33139 = F::cast_from(2.0_f64) * t1983 * t33137;
    let t33142 = t24999 * t1873;
    let t33144 = t33085 * t1873;
    (t33131, t33133, t33134, t33136, t33137, t33139, t33142, t33144)
}
