//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2706/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2706(t19815: f64, t3789: f64, t40159: f64, t6390: f64, t236: f64, t240: f64, t3869: f64, t247: f64, t5249: f64, t3798: f64, t1354: f64, t40130: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t57033 = t19815 * t3789;
    let t57041 = t40159 * t6390;
    let t57043 = t236 * t240;
    let t57044 = t57043 * t3869;
    let t57046 = t247 * t5249;
    let t57056 = t19815 * t3798;
    let t57057 = t57056 * t1354;
    let t57071 = t40130 * t6390;
    (t57033, t57041, t57043, t57044, t57046, t57057, t57071)
}
