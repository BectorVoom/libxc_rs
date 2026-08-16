//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1118/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1118(t21476: f64, t2537: f64, t7313: f64, t22238: f64, t7301: f64, t9647: f64, t1841: f64, t9649: f64, t23021: f64, t2558: f64, t9652: f64, t1843: f64, t22045: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29310 = 0.1281754371690370714e-2_f64 * t21476 * t2537 * t7313;
    let t29324 = 0.4486140300916297499e-2_f64 * t9647 * t22238 * t7301;
    let t29349 = 0.51270174867614828559e-2_f64 * t1841 * t9649;
    let t29354 = 0.64087718584518535698e-3_f64 * t9647 * t23021 * t2558;
    let t29434 = 0.34180116578409885706e-2_f64 * t1841 * t9652;
    let t29437 = 0.1281754371690370714e-2_f64 * t21476 * t1843 * t22045;
    (t29310, t29324, t29349, t29354, t29434, t29437)
}
