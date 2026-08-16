//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta803 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2917;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2918;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2919;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2920;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2921;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2922;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta803(t11468: f64, t15541: f64, t981: f64, t11591: f64, t4734: f64, t11602: f64, t4719: f64, t15543: f64, t3022: f64, t15547: f64, t3034: f64, t11610: f64, t15494: f64, t300: f64, t983: f64, t52516: f64, t52647: f64, t52650: f64, t52652: f64, t11223: f64, t379: f64, t51973: f64, t41361: f64, t41363: f64, t41369: f64, t42013: f64, t51849: f64, t51853: f64, t51858: f64, t51863: f64, t51867: f64, t51871: f64, t51875: f64, t51961: f64, t51965: f64, t51967: f64, t51971: f64, t51978: f64, t52028: f64, t52031: f64, t52033: f64, t52035: f64, t41308: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41365: f64, t41367: f64, t52037: f64, t52039: f64, t52041: f64, t52045: f64, t52047: f64, t52049: f64, t52051: f64, t52054: f64, t52057: f64, t52060: f64, t52063: f64, t52112: f64, t341: f64, t4930: f64, t989: f64, t1079: f64, t1097: f64, t11183: f64, t11184: f64, t11187: f64, t11210: f64, t11214: f64, t11902: f64, t16243: f64, t16249: f64, t16255: f64, t16292: f64, t16312: f64, t16313: f64, t16314: f64, t16322: f64, t16362: f64, t16591: f64, t16597: f64, t1680: f64, t3043: f64, t3047: f64, t3076: f64, t3261: f64, t3264: f64, t3326: f64, t386: f64, t43656: f64, t4743: f64, t4747: f64, t4758: f64, t4773: f64, t4932: f64, t4947: f64, t995: f64, t999: f64, t11199: f64, t1646: f64, t378: f64, t1072: f64, t994: f64, t3046: f64, t11174: f64, t11190: f64, t11203: f64, t11224: f64, t11804: f64, t12034: f64, t12039: f64, t16275: f64, t16295: f64, t16302: f64, t16318: f64, t16333: f64, t16344: f64, t16352: f64, t1647: f64, t1652: f64, t16605: f64, t1696: f64, t19428: f64, t3052: f64, t3063: f64, t3067: f64, t3271: f64, t33768: f64, t42038: f64, t42044: f64, t42052: f64, t4941: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52910, t52912, t52914, t52916, t52918, t52920) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2917(t11468, t15541, t981, t11591, t4734, t11602, t4719, t15543, t3022, t15547, t3034, t11610);
        let (t52923, t52924) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2918(t15494, t300, t983, t52516, t52647, t52650, t52652, t52910, t52912, t52914, t52916, t52918, t52920);
        let (t52927, t52954) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2919(t11223, t379, t51973, t41361, t41363, t41369, t42013, t51849, t51853, t51858, t51863, t51867, t51871, t51875, t51961, t51965, t51967, t51971, t51978, t52028, t52031, t52033);
        let t52975 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2920(t52035, t41308, t41330, t41332, t41334, t41336, t41365, t41367, t52037, t52039, t52041, t52045, t52047, t52049, t52051, t52054, t52057, t52060, t52063, t52112);
        let (t52977, t53011) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2921(t341, t52954, t52975, t4930, t989, t1079, t1097, t11183, t11184, t11187, t11210, t11214, t11902, t16243, t16249, t16255, t16292, t16312, t16313, t16314, t16322, t16362, t16591, t16597, t1680, t3043, t3047, t3076, t3261, t3264, t3326, t386, t43656, t4743, t4747, t4758, t4773, t4932, t4947, t52927, t995, t999);
        let (t53014, t53056) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2922(t11199, t1646, t378, t1072, t994, t3046, t379, t11174, t11187, t11190, t11203, t11224, t11804, t12034, t12039, t16249, t16275, t16295, t16302, t16312, t16318, t16322, t16333, t16344, t16352, t1647, t1652, t16605, t1696, t19428, t3052, t3063, t3067, t3271, t33768, t42038, t42044, t42052, t4747, t4941);
    (t52910, t52912, t52914, t52916, t52918, t52920, t52923, t52924, t52977, t53011, t53014, t53056)
}
