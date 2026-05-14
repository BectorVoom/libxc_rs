//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 679/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk679<F: Float>(t12871: F, t2859: F, t3085: F, t8124: F, t1445: F, t4527: F, t12806: F, t1562: F, t1531: F, t2876: F, t3159: F, t10485: F, t3377: F, t2778: F, t574: F, t2787: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12873 = 0.10725146985555128001e1 * t2859 * t12871;
    let t12874 = t8124 * t3085;
    let t12875 = t1445 * t12874;
    let t12877 = 0.27606906686822939767e2 * t4527 * t12875;
    let t12878 = t1445 * t12806;
    let t12880 = 0.62115540045351614476e2 * t1562 * t12878;
    let t12881 = t2876 * t1531;
    let t12883 = 0.25025342966295298669e1 * t3159 * t12881;
    let t12884 = t10485 * t3377;
    let t12886 = t2778 * t3085;
    let t12887 = t1445 * t12886;
    let t12889 = 0.92023022289409799224e1 * t574 * t12887;
    let t12890 = t2787 * t3085;
    (t12873, t12874, t12875, t12877, t12878, t12880, t12881, t12883, t12884, t12886, t12887, t12889, t12890)
}
