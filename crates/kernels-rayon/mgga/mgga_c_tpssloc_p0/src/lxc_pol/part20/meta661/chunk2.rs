//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2478/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2478(t11286: f64, t4869: f64, t1703: f64, t43700: f64, t11190: f64, t1670: f64, t11407: f64, t3242: f64, t457: f64, t45971: f64, t48140: f64, t2394: f64, t4734: f64) -> (f64, f64, f64, f64, f64) {
    let t50816 = 0.10254018858216406658e4_f64 * t4869 * t11286;
    let t50818 = 0.5848223622634646207e0_f64 * t43700 * t1703;
    let t50819 = t11190 * t1670;
    let t50821 = 0.2894756309764656312e3_f64 * t50819 * t11407;
    let t50822 = t457 * t3242;
    let t50824 = t48140 * t50822 * t45971;
    let t50826 = t2394 * t4734;
    (t50816, t50818, t50821, t50824, t50826)
}
