//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1181/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1181<F: Float>(t26654: F, t838: F, t26633: F, t26652: F, t26420: F, t27731: F, t27733: F, t27735: F, t27737: F, t27739: F, t27744: F, t27747: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t93826 = t838 * t26654;
    let t93848 = F::cast_from(3.0_f64) * t26633;
    let t93849 = F::cast_from(3.0_f64) * t26652;
    let t93852 = F::cast_from(12.0_f64) * t26420;
    let t95270 = t27731 / F::cast_from(8.0_f64);
    let t95271 = F::cast_from(2.0_f64) * t27733;
    let t95272 = t27735 / F::cast_from(8.0_f64);
    let t95273 = t27737 / F::cast_from(8.0_f64);
    let t95274 = t27739 / F::cast_from(8.0_f64);
    let t95276 = t27744 / F::cast_from(8.0_f64);
    let t95278 = t27747 / F::cast_from(8.0_f64);
    (t93826, t93848, t93849, t93852, t95270, t95271, t95272, t95273, t95274, t95276, t95278)
}
