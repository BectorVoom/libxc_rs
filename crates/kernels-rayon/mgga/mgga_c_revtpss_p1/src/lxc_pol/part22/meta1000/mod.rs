//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1000 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3398;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3399;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3400;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3401;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3402;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3403;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3404;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3405;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1000(t52126: f64, t52128: f64, t63447: f64, t63451: f64, t63453: f64, t63457: f64, t63459: f64, t63519: f64, t63522: f64, t63525: f64, t63528: f64, t63531: f64, t63533: f64, t63536: f64, t63538: f64, t41441: f64, t63462: f64, t63464: f64, t63541: f64, t63543: f64, t63545: f64, t63547: f64, t63549: f64, t63551: f64, t63554: f64, t63557: f64, t63560: f64, t63563: f64, t63566: f64, t63568: f64, t63700: f64, t63715: f64, t63731: f64, t63747: f64, t63764: f64, t63780: f64, t964: f64, t973: f64, t981: f64, t11465: f64, t3015: f64, t6205: f64, t1100: f64, t5019: f64, t18898: f64, t41813: f64, t19023: f64, t3022: f64, t41520: f64, t51967: f64, t63274: f64, t63276: f64, t63278: f64, t63281: f64, t63285: f64, t63290: f64, t63293: f64, t63299: f64, t63304: f64, t63308: f64, t41361: f64, t41363: f64, t51973: f64, t51978: f64, t63325: f64, t63328: f64, t63336: f64, t63338: f64, t63340: f64, t63342: f64, t63346: f64, t63351: f64, t63355: f64, t52033: f64, t52035: f64, t52037: f64, t52039: f64, t52041: f64, t52045: f64, t63359: f64, t63361: f64, t63366: f64, t63369: f64, t63371: f64, t63374: f64, t41330: f64, t41332: f64, t52047: f64, t52049: f64, t52051: f64, t63399: f64, t324: f64, t300: f64, t11506: f64, t15542: f64, t15566: f64, t19153: f64, t3329: f64, t5023: f64, t63673: f64, t63676: f64, t63679: f64, t63681: f64, t63683: f64, t63685: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t63797 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3398(t52126, t52128, t63447, t63451, t63453, t63457, t63459, t63519, t63522, t63525, t63528, t63531, t63533, t63536, t63538);
        let t63813 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3399(t41441, t63462, t63464, t63541, t63543, t63545, t63547, t63549, t63551, t63554, t63557, t63560, t63563, t63566, t63568);
        let (t63816, t63820, t63826) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3400(t63700, t63715, t63731, t63747, t63764, t63780, t63797, t63813, t964, t973, t981, t11465, t3015, t6205);
        let (t63827, t63833, t63835, t63847) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3401(t1100, t5019, t18898, t41813, t981, t19023, t3022, t41520, t51967, t63274, t63276, t63278, t63281, t63285, t63290, t63293, t63299, t63304, t63308);
        let t63861 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3402(t41361, t41363, t51973, t51978, t63325, t63328, t63336, t63338, t63340, t63342, t63346, t63351, t63355);
        let t63875 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3403(t52033, t52035, t52037, t52039, t52041, t52045, t63359, t63361, t63366, t63369, t63371, t63374);
        let t63889 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3404(t41330, t41332, t52047, t52049, t52051, t63399, t63447, t63451, t63453, t63457, t63459, t63462, t63464);
        let (t63892, t63894, t63898, t63899) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3405(t324, t63847, t63861, t63875, t63889, t300, t11506, t15542, t6205, t981, t15566, t19153, t3329, t5023, t63673, t63676, t63679, t63681, t63683, t63685, t63820, t63826, t63827, t63833, t63835);
    (t63816, t63820, t63826, t63833, t63835, t63892, t63894, t63898, t63899)
}
