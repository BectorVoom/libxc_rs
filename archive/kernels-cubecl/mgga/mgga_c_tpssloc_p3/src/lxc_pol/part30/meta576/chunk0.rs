//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1951/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1951<F: Float>(t1484: F, t1649: F, t28: F, t5544: F, t5664: F, t1530: F, t5660: F, t1877: F, t1915: F, t22959: F, t23295: F, t2522: F, t25358: F, t28448: F, t28765: F, t28771: F, t4314: F, t5966: F, t6670: F, t7541: F, t7649: F, t7656: F) -> (F, F, F, F, F, F) {
    let t28774 = t1649 * t1484;
    let t28778 = t28 * t5544;
    let t28789 = t28 * t5664;
    let t28792 = t1649 * t1530;
    let t28795 = t28 * t5660;
    let t28802 = F::cast_from(3.0_f64) * t4314 * t28765 + F::cast_from(3.0_f64) * t2522 * t7541 * t7649 - F::cast_from(3.0_f64) * t22959 * t28771 + F::cast_from(3.0_f64) * t2522 * t1915 * t28774 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t28778 + t1877 * t28448 * t28 / F::cast_from(2.0_f64) - t1877 * t25358 * t7656 + t1877 * t7541 * t1649 + t1877 * t23295 * t28789 - t1877 * t6670 * t28792 - t1877 * t6670 * t28795 / F::cast_from(2.0_f64) + t1877 * t1915 * t5966 / F::cast_from(2.0_f64);
    (t28774, t28778, t28789, t28792, t28795, t28802)
}
