//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta804 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2923;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2924;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2925;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2926;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2927;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta804<F: Float>(t11213: F, t1678: F, t3059: F, t4772: F, t16237: F, t342: F, t1000: F, t1073: F, t1076: F, t1079: F, t1097: F, t11121: F, t11122: F, t11177: F, t11184: F, t11195: F, t11201: F, t11214: F, t11220: F, t12043: F, t12173: F, t12178: F, t15886: F, t16371: F, t1651: F, t1652: F, t16600: F, t16603: F, t1695: F, t1696: F, t19428: F, t3075: F, t3269: F, t3326: F, t42107: F, t43687: F, t43696: F, t43707: F, t4758: F, t4764: F, t4778: F, t5015: F, t5016: F, t995: F, t996: F, t11120: F, t1071: F, t4743: F, t1078: F, t994: F, t11200: F, t11128: F, t11173: F, t11178: F, t11183: F, t11190: F, t11203: F, t11206: F, t12039: F, t12177: F, t16243: F, t16284: F, t16312: F, t16313: F, t16333: F, t16597: F, t16604: F, t3063: F, t3066: F, t3067: F, t3270: F, t43642: F, t4752: F, t4941: F, t3056: F, t4742: F, t378: F, t379: F, t3043: F, t3259: F, t4746: F, t11224: F, t11804: F, t15579: F, t16239: F, t16305: F, t16318: F, t16328: F, t16344: F, t16352: F, t16374: F, t16592: F, t3047: F, t3058: F, t3060: F, t3076: F, t3264: F, t3325: F, t33754: F, t989: F, t15885: F, t993: F, t51973: F, t41361: F, t41363: F, t41369: F, t42078: F, t51849: F, t51853: F, t51858: F, t51863: F, t51867: F, t51871: F, t51875: F, t51961: F, t51965: F, t51967: F, t51971: F, t51978: F, t52028: F, t52031: F, t52033: F, t52035: F, t52037: F, t41308: F, t41330: F, t41332: F, t41334: F, t41336: F, t41365: F, t41367: F, t52039: F, t52041: F, t52045: F, t52047: F, t52049: F, t52051: F, t52054: F, t52057: F, t52060: F, t52063: F, t52112: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t53089, t53107) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2923::<F>(t11213, t1678, t3059, t4772, t16237, t342, t1000, t1073, t1076, t1079, t1097, t11121, t11122, t11177, t11184, t11195, t11201, t11214, t11220, t12043, t12173, t12178, t15886, t16371, t1651, t1652, t16600, t16603, t1695, t1696, t19428, t3075, t3269, t3326, t42107, t43687, t43696, t43707, t4758, t4764, t4778, t5015, t5016, t995, t996);
        let (t53108, t53163) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2924::<F>(t11120, t1695, t1071, t4743, t1078, t4772, t16237, t994, t11200, t1678, t1000, t1076, t1079, t1097, t11121, t11128, t11173, t11178, t11183, t11190, t11203, t11206, t12039, t12177, t12178, t16243, t16284, t16312, t16313, t16333, t16597, t16603, t16604, t3063, t3066, t3067, t3270, t3326, t43642, t4752, t4758, t4764, t4941, t5015, t995);
        let (t53166, t53192, t53217) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2925::<F>(t3056, t4742, t378, t11200, t379, t1678, t3043, t3075, t4772, t3259, t4746, t1000, t1079, t1097, t11214, t11224, t11804, t12043, t12177, t15579, t16239, t16284, t16305, t16313, t16318, t16328, t16344, t16352, t16374, t16592, t3047, t3058, t3060, t3063, t3067, t3076, t3264, t3269, t3270, t3325, t33754, t4941, t989, t995, t996);
        let (t53222, t53223, t53251) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2926::<F>(t15885, t993, t378, t51973, t41361, t41363, t41369, t42078, t51849, t51853, t51858, t51863, t51867, t51871, t51875, t51961, t51965, t51967, t51971, t51978, t52028, t52031, t52033);
        let t53272 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2927::<F>(t52035, t52037, t41308, t41330, t41332, t41334, t41336, t41365, t41367, t52039, t52041, t52045, t52047, t52049, t52051, t52054, t52057, t52060, t52063, t52112);
    (t53089, t53107, t53108, t53163, t53166, t53192, t53217, t53222, t53223, t53251, t53272)
}
