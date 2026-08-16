//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta681 (260520-c91 hierarchical CSE).
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
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2138;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2139;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2140;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2141;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2142;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2143;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2144;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2145;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2146;
use chunk9::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2147;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta681(t1351: f64, t1992: f64, t550: f64, t6434: f64, t6976: f64, t22704: f64, t22705: f64, t28167: f64, t26331: f64, t26421: f64, t26446: f64, t5187: f64, t22897: f64, t3792: f64, t57607: f64, t6378: f64, t6990: f64, t81039: f64, t81047: f64, t90845: f64, t90860: f64, t90865: f64, t90867: f64, t93538: f64, t96962: f64, t96967: f64, t96972: f64, t96976: f64, t96979: f64, t19745: f64, t81027: f64, t12369: f64, t19743: f64, t22633: f64, t562: f64, t6330: f64, t1307: f64, t90591: f64, t20018: f64, t57499: f64, t28163: f64, t57618: f64, t1332: f64, t19805: f64, t2013: f64, t28156: f64, t81061: f64, t81066: f64, t81073: f64, t81075: f64, t81076: f64, t90899: f64, t90913: f64, t93563: f64, t22881: f64, t6347: f64, t6637: f64, t6888: f64, t19631: f64, t6968: f64, t28130: f64, t81228: f64, t19748: f64, t28134: f64, t80798: f64, t1985: f64, t1998: f64, t20009: f64, t214: f64, t1352: f64, t1799: f64, t90809: f64, t26395: f64, t22892: f64, t22893: f64, t28148: f64, t1336: f64, t19732: f64, t19815: f64, t28178: f64, t3777: f64, t6987: f64, t6988: f64, t81080: f64, t90957: f64, t90962: f64, t90964: f64, t19761: f64, t1825: f64, t90754: f64, t90818: f64, t5287: f64, t22751: f64, t28149: f64, t19740: f64, t28139: f64, t28159: f64, t6897: f64, t794: f64, t19763: f64, t19735: f64, t22873: f64, t26403: f64, t26459: f64, t5234: f64, t5334: f64, t6388: f64, t6415: f64, t81105: f64, t90971: f64, t90984: f64, t90988: f64, t93595: f64, t19739: f64, t3807: f64, t28131: f64, t81159: f64, t552: f64, t96964: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96986, t96989, t96993) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2138(t1351, t1992, t550, t6434, t6976, t22704, t22705, t28167, t26331, t26421, t26446, t5187);
        let t96999 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2139(t1992, t22897, t3792, t57607, t6378, t6990, t81039, t81047, t90845, t90860, t90865, t90867, t93538, t96962, t96967, t96972, t96976, t96979, t96986, t96989, t96993);
        let (t97002, t97007, t97011, t97014) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2140(t19745, t1992, t81027, t12369, t19743, t22633, t22897, t562, t6330, t1307, t26446, t90591);
        let t97032 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2141(t1992, t20018, t6976, t550, t57499, t22704, t22705, t28163, t57618, t1332, t19805, t2013, t28156, t81061, t81066, t81073, t81075, t81076, t90899, t90913, t93563, t97002, t97007, t97014);
        let (t97036, t97040, t97043, t97046) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2142(t22881, t6347, t6637, t6888, t19631, t6968, t22705, t28130, t81228, t19748, t1992, t22897);
        let (t97049, t97055, t97059, t97063) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2143(t22704, t28134, t80798, t1985, t1998, t20009, t214, t1352, t26331, t6976, t97011, t1799, t6637, t6888, t90809);
        let t97075 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2144(t26395, t5187, t6637, t6888, t22892, t22893, t28148, t1336, t19732, t19815, t28178, t3777, t6987, t6988, t81080, t90957, t90962, t90964, t97036, t97040, t97043, t97046, t97049, t97055, t97059, t97063);
        let (t97079, t97083, t97087, t97091, t97095, t97106) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2145(t19761, t1992, t6976, t1825, t22633, t90754, t90818, t26421, t5287, t22751, t28149, t19740, t22897);
        let t97116 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2146(t22751, t28139, t28159, t6897, t794, t19763, t1992, t6976, t1336, t19735, t22873, t26403, t26459, t5234, t5334, t6388, t6415, t81105, t90971, t90984, t90988, t93595, t97079, t97083, t97087, t97091, t97095, t97106);
        let (t97119, t97124, t97129, t97135) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2147(t19739, t22633, t3807, t6976, t28131, t81159, t552, t6434, t1307, t6637, t6888, t26331, t26446, t96964);
    (t96999, t97032, t97075, t97116, t97119, t97124, t97129, t97135)
}
