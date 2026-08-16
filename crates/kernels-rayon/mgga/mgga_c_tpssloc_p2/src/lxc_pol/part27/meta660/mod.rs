//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta660 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2305;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2306;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2307;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2308;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2309;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2310;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2311;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2312;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2313;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta660(t16052: f64, t1992: f64, t22897: f64, t26392: f64, t80670: f64, t16419: f64, t6976: f64, t22705: f64, t26422: f64, t81228: f64, t16040: f64, t22633: f64, t3807: f64, t54854: f64, t550: f64, t26331: f64, t26421: f64, t26446: f64, t3719: f64, t22704: f64, t26466: f64, t81022: f64, t90806: f64, t90807: f64, t90812: f64, t90816: f64, t90821: f64, t90825: f64, t90829: f64, t90832: f64, t26461: f64, t26433: f64, t6883: f64, t22716: f64, t7741: f64, t1834: f64, t3791: f64, t81039: f64, t54840: f64, t54883: f64, t81061: f64, t3793: f64, t16041: f64, t5336: f64, t80798: f64, t22724: f64, t26436: f64, t81037: f64, t81041: f64, t81043: f64, t81047: f64, t81050: f64, t81066: f64, t1307: f64, t1352: f64, t16037: f64, t26423: f64, t81159: f64, t215: f64, t22839: f64, t562: f64, t80854: f64, t16226: f64, t22685: f64, t26395: f64, t3734: f64, t6637: f64, t81080: f64, t16125: f64, t3856: f64, t12267: f64, t1336: f64, t22873: f64, t5287: f64, t7745: f64, t81069: f64, t81073: f64, t81075: f64, t81076: f64, t81083: f64, t81099: f64, t1824: f64, t6955: f64, t2006: f64, t5286: f64, t1338: f64, t26328: f64, t26462: f64, t6914: f64, t26414: f64, t26415: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90835, t90837, t90840, t90845, t90848) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2305(t16052, t1992, t22897, t26392, t80670, t16419, t6976, t22705, t26422, t81228, t16040, t22633, t3807);
        let t90861 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2306(t1992, t54854, t550, t6976, t26331, t26421, t26446, t3719, t22704, t22705, t26466, t81022, t90806, t90807, t90812, t90816, t90821, t90825, t90829, t90832, t90835, t90837, t90840, t90845, t90848);
        let (t90865, t90867, t90868, t90870, t90873) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2307(t22704, t22705, t26461, t26433, t6883, t22716, t7741, t1834, t3791, t1992, t550, t6976);
        let (t90876, t90883, t90887, t90889, t90892, t90895) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2308(t81039, t1992, t54840, t550, t6976, t54883, t81061, t22633, t22897, t26421, t3793, t16041);
        let t90902 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2309(t22704, t5336, t80798, t22724, t26436, t81037, t81041, t81043, t81047, t81050, t90865, t90867, t90868, t90873, t90876, t90883, t90887, t90889, t90892, t90895);
        let (t90903, t90907, t90910, t90913, t90914) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2310(t81066, t1307, t1352, t1834, t22633, t6976, t16037, t1992, t22897, t26423, t81159, t215, t22839);
        let (t90915, t90917, t90921, t90925, t90929) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2311(t562, t80854, t16226, t90914, t22685, t26395, t3734, t6637, t81080, t16125, t1992, t6976);
        let t90939 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2312(t22633, t26421, t3856, t6976, t12267, t1336, t22873, t5287, t7745, t81069, t81073, t81075, t81076, t81083, t81099, t90903, t90907, t90910, t90913, t90917, t90921, t90925, t90929);
        let (t90942, t90946, t90952, t90957, t90962, t90963) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2313(t1824, t6955, t2006, t5286, t1338, t26328, t26462, t6914, t22705, t26414, t81228, t26415, t81159);
    (t90861, t90870, t90902, t90915, t90939, t90942, t90946, t90952, t90957, t90962, t90963)
}
