//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta804 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2923;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2924;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2925;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2926;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2927;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta804(t11213: f64, t1678: f64, t3059: f64, t4772: f64, t16237: f64, t342: f64, t1000: f64, t1073: f64, t1076: f64, t1079: f64, t1097: f64, t11121: f64, t11122: f64, t11177: f64, t11184: f64, t11195: f64, t11201: f64, t11214: f64, t11220: f64, t12043: f64, t12173: f64, t12178: f64, t15886: f64, t16371: f64, t1651: f64, t1652: f64, t16600: f64, t16603: f64, t1695: f64, t1696: f64, t19428: f64, t3075: f64, t3269: f64, t3326: f64, t42107: f64, t43687: f64, t43696: f64, t43707: f64, t4758: f64, t4764: f64, t4778: f64, t5015: f64, t5016: f64, t995: f64, t996: f64, t11120: f64, t1071: f64, t4743: f64, t1078: f64, t994: f64, t11200: f64, t11128: f64, t11173: f64, t11178: f64, t11183: f64, t11190: f64, t11203: f64, t11206: f64, t12039: f64, t12177: f64, t16243: f64, t16284: f64, t16312: f64, t16313: f64, t16333: f64, t16597: f64, t16604: f64, t3063: f64, t3066: f64, t3067: f64, t3270: f64, t43642: f64, t4752: f64, t4941: f64, t3056: f64, t4742: f64, t378: f64, t379: f64, t3043: f64, t3259: f64, t4746: f64, t11224: f64, t11804: f64, t15579: f64, t16239: f64, t16305: f64, t16318: f64, t16328: f64, t16344: f64, t16352: f64, t16374: f64, t16592: f64, t3047: f64, t3058: f64, t3060: f64, t3076: f64, t3264: f64, t3325: f64, t33754: f64, t989: f64, t15885: f64, t993: f64, t51973: f64, t41361: f64, t41363: f64, t41369: f64, t42078: f64, t51849: f64, t51853: f64, t51858: f64, t51863: f64, t51867: f64, t51871: f64, t51875: f64, t51961: f64, t51965: f64, t51967: f64, t51971: f64, t51978: f64, t52028: f64, t52031: f64, t52033: f64, t52035: f64, t52037: f64, t41308: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41365: f64, t41367: f64, t52039: f64, t52041: f64, t52045: f64, t52047: f64, t52049: f64, t52051: f64, t52054: f64, t52057: f64, t52060: f64, t52063: f64, t52112: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53089, t53107) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2923(t11213, t1678, t3059, t4772, t16237, t342, t1000, t1073, t1076, t1079, t1097, t11121, t11122, t11177, t11184, t11195, t11201, t11214, t11220, t12043, t12173, t12178, t15886, t16371, t1651, t1652, t16600, t16603, t1695, t1696, t19428, t3075, t3269, t3326, t42107, t43687, t43696, t43707, t4758, t4764, t4778, t5015, t5016, t995, t996);
        let (t53108, t53163) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2924(t11120, t1695, t1071, t4743, t1078, t4772, t16237, t994, t11200, t1678, t1000, t1076, t1079, t1097, t11121, t11128, t11173, t11178, t11183, t11190, t11203, t11206, t12039, t12177, t12178, t16243, t16284, t16312, t16313, t16333, t16597, t16603, t16604, t3063, t3066, t3067, t3270, t3326, t43642, t4752, t4758, t4764, t4941, t5015, t995);
        let (t53166, t53192, t53217) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2925(t3056, t4742, t378, t11200, t379, t1678, t3043, t3075, t4772, t3259, t4746, t1000, t1079, t1097, t11214, t11224, t11804, t12043, t12177, t15579, t16239, t16284, t16305, t16313, t16318, t16328, t16344, t16352, t16374, t16592, t3047, t3058, t3060, t3063, t3067, t3076, t3264, t3269, t3270, t3325, t33754, t4941, t989, t995, t996);
        let (t53222, t53223, t53251) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2926(t15885, t993, t378, t51973, t41361, t41363, t41369, t42078, t51849, t51853, t51858, t51863, t51867, t51871, t51875, t51961, t51965, t51967, t51971, t51978, t52028, t52031, t52033);
        let t53272 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2927(t52035, t52037, t41308, t41330, t41332, t41334, t41336, t41365, t41367, t52039, t52041, t52045, t52047, t52049, t52051, t52054, t52057, t52060, t52063, t52112);
    (t53089, t53107, t53108, t53163, t53166, t53192, t53217, t53222, t53223, t53251, t53272)
}
