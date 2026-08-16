//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta918 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2959;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2960;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2961;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2962;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2963;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta918<F: Float>(t19150: F, t4719: F, t19167: F, t4724: F, t981: F, t19471: F, t18899: F, t23451: F, t41224: F, t23648: F, t4733: F, t23568: F, t3022: F, t78446: F, t78449: F, t78451: F, t78456: F, t78458: F, t24186: F, t3336: F, t11249: F, t23640: F, t15926: F, t19976: F, t1651: F, t606: F, t11703: F, t11859: F, t16052: F, t16081: F, t16089: F, t16095: F, t18908: F, t19634: F, t19758: F, t19831: F, t20096: F, t20101: F, t23481: F, t23900: F, t23992: F, t2852: F, t2857: F, t3091: F, t3092: F, t3117: F, t4181: F, t43254: F, t4757: F, t4786: F, t4891: F, t4896: F, t4902: F, t54500: F, t54570: F, t6100: F, t6258: F, t65288: F, t65292: F, t65298: F, t66766: F, t67725: F, t67790: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t78460, t78463, t78465, t78469, t78472, t78474) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2959::<F>(t19150, t4719, t19167, t4724, t981, t19471, t18899, t23451, t41224, t23648, t4733, t23568, t3022);
        let (t78475, t78478) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2960::<F>(t78446, t78449, t78451, t78456, t78458, t78460, t78463, t78465, t78469, t78472, t78474, t24186, t3336);
        let t78496 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2961::<F>(t11249, t23640);
        let (t78512, t78524) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2962::<F>(t15926, t19976, t1651, t606);
        let t78545 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2963::<F>(t11703, t11859, t16052, t16081, t16089, t16095, t18908, t19634, t19758, t19831, t20096, t20101, t23481, t23900, t23992, t2852, t2857, t3091, t3092, t3117, t4181, t43254, t4757, t4786, t4891, t4896, t4902, t54500, t54570, t6100, t6258, t65288, t65292, t65298, t66766, t67725, t67790, t78496, t78512, t78524);
    (t78460, t78463, t78465, t78469, t78472, t78474, t78475, t78478, t78496, t78524, t78545)
}
