//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2326/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2326<F: Float>(t1649: F, t4119: F, t23788: F, t67123: F, t1081: F, t5660: F, t5544: F, t16662: F, t28: F, t5527: F, t1877: F, t1915: F, t22959: F, t2522: F, t25901: F, t25905: F, t25928: F, t25938: F, t28448: F, t28764: F, t28765: F, t4314: F, t46341: F, t5966: F, t6666: F, t6670: F, t6841: F, t7541: F, t98027: F) -> F {
    let t100718 = t1649 * t4119;
    let t100731 = t23788 * t67123;
    let t100734 = t1081 * t5660;
    let t100743 = t1081 * t5544;
    let t100747 = t28 * t16662;
    let t100759 = t1081 * t5527;
    let t100763 = F::cast_from(3.0_f64) * t2522 * t1915 * t100718 + F::cast_from(3.0_f64) * t2522 * t7541 * t25901 + F::cast_from(3.0_f64) * t4314 * t6666 * t28764 + F::cast_from(3.0_f64) * t2522 * t7541 * t25905 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t22959 * t100731 - t1877 * t6670 * t100734 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t28448 * t6841 + F::cast_from(2.0_f64) * t98027 * t25928 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t100743 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t100747 + F::cast_from(3.0_f64) * t46341 * t28765 + t1877 * t6666 * t5966 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) * t2522 * t7541 * t25938 + F::cast_from(3.0_f64) * t4314 * t1915 * t100759;
    t100763
}
