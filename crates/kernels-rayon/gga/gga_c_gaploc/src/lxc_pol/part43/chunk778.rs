//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 778/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk778(t12206: f64, t783: f64, t12161: f64, t835: f64, t325: f64, t1858: f64, t3720: f64, t38907: f64, t739: f64, t2089: f64, t7290: f64, t321: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38950 = t12206 * t783;
    let t38961 = t835 * t12161;
    let t38974 = t325 * t12161;
    let t39002 = t1858 * t3720;
    let t39022 = t739 * t38907;
    let t39027 = t2089 * t12161;
    let t39040 = t7290 * t38907;
    let t39048 = t321 * t3720;
    (t38950, t38961, t38974, t39002, t39022, t39027, t39040, t39048)
}
