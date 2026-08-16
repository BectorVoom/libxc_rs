//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta918 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2959;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2960;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2961;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2962;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2963;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta918(t19150: f64, t4719: f64, t19167: f64, t4724: f64, t981: f64, t19471: f64, t18899: f64, t23451: f64, t41224: f64, t23648: f64, t4733: f64, t23568: f64, t3022: f64, t78446: f64, t78449: f64, t78451: f64, t78456: f64, t78458: f64, t24186: f64, t3336: f64, t11249: f64, t23640: f64, t15926: f64, t19976: f64, t1651: f64, t606: f64, t11703: f64, t11859: f64, t16052: f64, t16081: f64, t16089: f64, t16095: f64, t18908: f64, t19634: f64, t19758: f64, t19831: f64, t20096: f64, t20101: f64, t23481: f64, t23900: f64, t23992: f64, t2852: f64, t2857: f64, t3091: f64, t3092: f64, t3117: f64, t4181: f64, t43254: f64, t4757: f64, t4786: f64, t4891: f64, t4896: f64, t4902: f64, t54500: f64, t54570: f64, t6100: f64, t6258: f64, t65288: f64, t65292: f64, t65298: f64, t66766: f64, t67725: f64, t67790: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t78460, t78463, t78465, t78469, t78472, t78474) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2959(t19150, t4719, t19167, t4724, t981, t19471, t18899, t23451, t41224, t23648, t4733, t23568, t3022);
        let (t78475, t78478) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2960(t78446, t78449, t78451, t78456, t78458, t78460, t78463, t78465, t78469, t78472, t78474, t24186, t3336);
        let t78496 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2961(t11249, t23640);
        let (t78512, t78524) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2962(t15926, t19976, t1651, t606);
        let t78545 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2963(t11703, t11859, t16052, t16081, t16089, t16095, t18908, t19634, t19758, t19831, t20096, t20101, t23481, t23900, t23992, t2852, t2857, t3091, t3092, t3117, t4181, t43254, t4757, t4786, t4891, t4896, t4902, t54500, t54570, t6100, t6258, t65288, t65292, t65298, t66766, t67725, t67790, t78496, t78512, t78524);
    (t78460, t78463, t78465, t78469, t78472, t78474, t78475, t78478, t78496, t78524, t78545)
}
