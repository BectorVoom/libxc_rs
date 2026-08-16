//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2478/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2478<F: Float>(t11286: F, t4869: F, t1703: F, t43700: F, t11190: F, t1670: F, t11407: F, t3242: F, t457: F, t45971: F, t48140: F, t2394: F, t4734: F) -> (F, F, F, F, F) {
    let t50816 = F::cast_from(0.10254018858216406658e4_f64) * t4869 * t11286;
    let t50818 = F::cast_from(0.5848223622634646207e0_f64) * t43700 * t1703;
    let t50819 = t11190 * t1670;
    let t50821 = F::cast_from(0.2894756309764656312e3_f64) * t50819 * t11407;
    let t50822 = t457 * t3242;
    let t50824 = t48140 * t50822 * t45971;
    let t50826 = t2394 * t4734;
    (t50816, t50818, t50821, t50824, t50826)
}
