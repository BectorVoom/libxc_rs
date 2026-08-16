//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta803 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2917;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2918;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2919;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2920;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2921;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2922;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta803<F: Float>(t11468: F, t15541: F, t981: F, t11591: F, t4734: F, t11602: F, t4719: F, t15543: F, t3022: F, t15547: F, t3034: F, t11610: F, t15494: F, t300: F, t983: F, t52516: F, t52647: F, t52650: F, t52652: F, t11223: F, t379: F, t51973: F, t41361: F, t41363: F, t41369: F, t42013: F, t51849: F, t51853: F, t51858: F, t51863: F, t51867: F, t51871: F, t51875: F, t51961: F, t51965: F, t51967: F, t51971: F, t51978: F, t52028: F, t52031: F, t52033: F, t52035: F, t41308: F, t41330: F, t41332: F, t41334: F, t41336: F, t41365: F, t41367: F, t52037: F, t52039: F, t52041: F, t52045: F, t52047: F, t52049: F, t52051: F, t52054: F, t52057: F, t52060: F, t52063: F, t52112: F, t341: F, t4930: F, t989: F, t1079: F, t1097: F, t11183: F, t11184: F, t11187: F, t11210: F, t11214: F, t11902: F, t16243: F, t16249: F, t16255: F, t16292: F, t16312: F, t16313: F, t16314: F, t16322: F, t16362: F, t16591: F, t16597: F, t1680: F, t3043: F, t3047: F, t3076: F, t3261: F, t3264: F, t3326: F, t386: F, t43656: F, t4743: F, t4747: F, t4758: F, t4773: F, t4932: F, t4947: F, t995: F, t999: F, t11199: F, t1646: F, t378: F, t1072: F, t994: F, t3046: F, t11174: F, t11190: F, t11203: F, t11224: F, t11804: F, t12034: F, t12039: F, t16275: F, t16295: F, t16302: F, t16318: F, t16333: F, t16344: F, t16352: F, t1647: F, t1652: F, t16605: F, t1696: F, t19428: F, t3052: F, t3063: F, t3067: F, t3271: F, t33768: F, t42038: F, t42044: F, t42052: F, t4941: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t52910, t52912, t52914, t52916, t52918, t52920) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2917::<F>(t11468, t15541, t981, t11591, t4734, t11602, t4719, t15543, t3022, t15547, t3034, t11610);
        let (t52923, t52924) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2918::<F>(t15494, t300, t983, t52516, t52647, t52650, t52652, t52910, t52912, t52914, t52916, t52918, t52920);
        let (t52927, t52954) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2919::<F>(t11223, t379, t51973, t41361, t41363, t41369, t42013, t51849, t51853, t51858, t51863, t51867, t51871, t51875, t51961, t51965, t51967, t51971, t51978, t52028, t52031, t52033);
        let t52975 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2920::<F>(t52035, t41308, t41330, t41332, t41334, t41336, t41365, t41367, t52037, t52039, t52041, t52045, t52047, t52049, t52051, t52054, t52057, t52060, t52063, t52112);
        let (t52977, t53011) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2921::<F>(t341, t52954, t52975, t4930, t989, t1079, t1097, t11183, t11184, t11187, t11210, t11214, t11902, t16243, t16249, t16255, t16292, t16312, t16313, t16314, t16322, t16362, t16591, t16597, t1680, t3043, t3047, t3076, t3261, t3264, t3326, t386, t43656, t4743, t4747, t4758, t4773, t4932, t4947, t52927, t995, t999);
        let (t53014, t53056) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2922::<F>(t11199, t1646, t378, t1072, t994, t3046, t379, t11174, t11187, t11190, t11203, t11224, t11804, t12034, t12039, t16249, t16275, t16295, t16302, t16312, t16318, t16322, t16333, t16344, t16352, t1647, t1652, t16605, t1696, t19428, t3052, t3063, t3067, t3271, t33768, t42038, t42044, t42052, t4747, t4941);
    (t52910, t52912, t52914, t52916, t52918, t52920, t52923, t52924, t52977, t53011, t53014, t53056)
}
