//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1014/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1014(t127: f64, t645: f64, t6856: f64, t22166: f64, t1928: f64, t6926: f64, t6931: f64, t2030: f64, t6938: f64, t6936: f64, t616: f64, t6877: f64, t6879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22168 = t6856 * t645 * t127;
    let t22169 = t22166 * t22168;
    let t22172 = t6926 * t1928;
    let t22173 = t6931 * t22172;
    let t22176 = t2030 * t6938;
    let t22178 = t6936 * t1928;
    let t22179 = t6931 * t22178;
    let t22187 = t6877 * t6879 * t616;
    (t22168, t22169, t22172, t22173, t22176, t22178, t22179, t22187)
}
