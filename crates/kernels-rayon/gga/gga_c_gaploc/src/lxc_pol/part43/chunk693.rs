//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 693/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk693(t12886: f64, t1445: f64, t574: f64, t2787: f64, t3085: f64, t597: f64, t12806: f64, t1457: f64, t4540: f64, t12766: f64, t1572: f64, t2877: f64, t3149: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12887 = t1445 * t12886;
    let t12889 = 0.92023022289409799224e1_f64 * t574 * t12887;
    let t12890 = t2787 * t3085;
    let t12891 = t1445 * t12890;
    let t12893 = 0.43710935587469654631e2_f64 * t597 * t12891;
    let t12894 = t1457 * t12806;
    let t12896 = 0.21450293971110256001e1_f64 * t4540 * t12894;
    let t12900 = t1457 * t12766;
    let t12902 = 0.71500979903700853338e0_f64 * t1572 * t12900;
    let t12909 = 0.35750489951850426669e0_f64 * t3149 * t2877;
    (t12887, t12889, t12890, t12891, t12893, t12894, t12896, t12900, t12902, t12909)
}
