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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3398;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3399;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3400;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3401;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3402;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3403;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3404;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3405;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1000<F: Float>(t52126: F, t52128: F, t63447: F, t63451: F, t63453: F, t63457: F, t63459: F, t63519: F, t63522: F, t63525: F, t63528: F, t63531: F, t63533: F, t63536: F, t63538: F, t41441: F, t63462: F, t63464: F, t63541: F, t63543: F, t63545: F, t63547: F, t63549: F, t63551: F, t63554: F, t63557: F, t63560: F, t63563: F, t63566: F, t63568: F, t63700: F, t63715: F, t63731: F, t63747: F, t63764: F, t63780: F, t964: F, t973: F, t981: F, t11465: F, t3015: F, t6205: F, t1100: F, t5019: F, t18898: F, t41813: F, t19023: F, t3022: F, t41520: F, t51967: F, t63274: F, t63276: F, t63278: F, t63281: F, t63285: F, t63290: F, t63293: F, t63299: F, t63304: F, t63308: F, t41361: F, t41363: F, t51973: F, t51978: F, t63325: F, t63328: F, t63336: F, t63338: F, t63340: F, t63342: F, t63346: F, t63351: F, t63355: F, t52033: F, t52035: F, t52037: F, t52039: F, t52041: F, t52045: F, t63359: F, t63361: F, t63366: F, t63369: F, t63371: F, t63374: F, t41330: F, t41332: F, t52047: F, t52049: F, t52051: F, t63399: F, t324: F, t300: F, t11506: F, t15542: F, t15566: F, t19153: F, t3329: F, t5023: F, t63673: F, t63676: F, t63679: F, t63681: F, t63683: F, t63685: F) -> (F, F, F, F, F, F, F, F, F) {
        let t63797 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3398::<F>(t52126, t52128, t63447, t63451, t63453, t63457, t63459, t63519, t63522, t63525, t63528, t63531, t63533, t63536, t63538);
        let t63813 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3399::<F>(t41441, t63462, t63464, t63541, t63543, t63545, t63547, t63549, t63551, t63554, t63557, t63560, t63563, t63566, t63568);
        let (t63816, t63820, t63826) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3400::<F>(t63700, t63715, t63731, t63747, t63764, t63780, t63797, t63813, t964, t973, t981, t11465, t3015, t6205);
        let (t63827, t63833, t63835, t63847) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3401::<F>(t1100, t5019, t18898, t41813, t981, t19023, t3022, t41520, t51967, t63274, t63276, t63278, t63281, t63285, t63290, t63293, t63299, t63304, t63308);
        let t63861 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3402::<F>(t41361, t41363, t51973, t51978, t63325, t63328, t63336, t63338, t63340, t63342, t63346, t63351, t63355);
        let t63875 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3403::<F>(t52033, t52035, t52037, t52039, t52041, t52045, t63359, t63361, t63366, t63369, t63371, t63374);
        let t63889 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3404::<F>(t41330, t41332, t52047, t52049, t52051, t63399, t63447, t63451, t63453, t63457, t63459, t63462, t63464);
        let (t63892, t63894, t63898, t63899) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3405::<F>(t324, t63847, t63861, t63875, t63889, t300, t11506, t15542, t6205, t981, t15566, t19153, t3329, t5023, t63673, t63676, t63679, t63681, t63683, t63685, t63820, t63826, t63827, t63833, t63835);
    (t63816, t63820, t63826, t63833, t63835, t63892, t63894, t63898, t63899)
}
