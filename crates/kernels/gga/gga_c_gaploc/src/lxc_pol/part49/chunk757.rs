//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 757/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk757<F: Float>(t1531: F, t2876: F, t3159: F, t10485: F, t3377: F, t2778: F, t3085: F, t1445: F, t574: F, t2787: F, t597: F, t12806: F, t1457: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12881 = t2876 * t1531;
    let t12883 = F::new(0.25025342966295298669e1) * t3159 * t12881;
    let t12884 = t10485 * t3377;
    let t12886 = t2778 * t3085;
    let t12887 = t1445 * t12886;
    let t12889 = F::new(0.92023022289409799224e1) * t574 * t12887;
    let t12890 = t2787 * t3085;
    let t12891 = t1445 * t12890;
    let t12893 = F::new(0.43710935587469654631e2) * t597 * t12891;
    let t12894 = t1457 * t12806;
    (t12881, t12883, t12884, t12886, t12887, t12889, t12890, t12891, t12893, t12894)
}
