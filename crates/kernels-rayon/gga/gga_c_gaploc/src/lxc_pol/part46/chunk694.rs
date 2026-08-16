//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 694/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk694(t12881: f64, t3159: f64, t10485: f64, t3377: f64, t2778: f64, t3085: f64, t1445: f64, t574: f64, t2787: f64, t597: f64, t12806: f64, t1457: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12883 = 0.25025342966295298669e1_f64 * t3159 * t12881;
    let t12884 = t10485 * t3377;
    let t12886 = t2778 * t3085;
    let t12887 = t1445 * t12886;
    let t12889 = 0.92023022289409799224e1_f64 * t574 * t12887;
    let t12890 = t2787 * t3085;
    let t12891 = t1445 * t12890;
    let t12893 = 0.43710935587469654631e2_f64 * t597 * t12891;
    let t12894 = t1457 * t12806;
    (t12883, t12884, t12886, t12887, t12889, t12890, t12891, t12893, t12894)
}
