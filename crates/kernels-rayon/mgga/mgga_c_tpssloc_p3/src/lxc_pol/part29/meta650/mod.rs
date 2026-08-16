//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta650 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2169;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2170;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2171;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2172;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2173;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2174;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta650(t1649: f64, t2553: f64, t12971: f64, t28: f64, t1081: f64, t4119: f64, t13191: f64, t25891: f64, t25927: f64, t57921: f64, t13471: f64, t1484: f64, t3231: f64, t86781: f64, t1877: f64, t1915: f64, t22959: f64, t23286: f64, t23290: f64, t25013: f64, t2522: f64, t25928: f64, t25945: f64, t6670: f64, t7649: f64, t86703: f64, t86734: f64, t86751: f64, t86757: f64, t87945: f64, t23788: f64, t86797: f64, t16596: f64, t83555: f64, t4303: f64, t40772: f64, t86717: f64, t23781: f64, t23807: f64, t23810: f64, t23813: f64, t25354: f64, t25358: f64, t25372: f64, t25892: f64, t25898: f64, t25905: f64, t4314: f64, t6666: f64, t6841: f64, t7541: f64, t81483: f64, t86740: f64, t86775: f64, t86835: f64, t87975: f64, t25365: f64, t1530: f64, t2749: f64, t57893: f64, t2752: f64, t13487: f64, t23295: f64, t23796: f64, t25901: f64, t25921: f64, t25930: f64, t25934: f64, t25938: f64, t47645: f64, t7650: f64, t7656: f64, t81525: f64, t89880: f64, t23858: f64, t7685: f64, t22607: f64, t7688: f64, t1390: f64, t16018: f64, t1983: f64, t6878: f64, t22574: f64, t56194: f64, t8643: f64, t12461: f64, t6995: f64, t26161: f64, t26163: f64, t22581: f64, t24987: f64, t7000: f64, t25985: f64, t6876: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t89881, t89888, t89892, t89896, t89904, t89907, t89911) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2169(t1649, t2553, t12971, t28, t1081, t4119, t13191, t25891, t25927, t57921, t13471, t1484, t3231);
        let t89920 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2170(t25927, t86781, t1877, t1915, t22959, t23286, t23290, t25013, t2522, t25928, t25945, t28, t6670, t7649, t86703, t86734, t86751, t86757, t87945, t89881, t89888, t89892, t89896, t89904, t89907, t89911);
        let t89957 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2171(t23788, t86797, t16596, t83555, t1081, t4303, t28, t40772, t86717, t1877, t22959, t23781, t23807, t23810, t23813, t25013, t2522, t25354, t25358, t25372, t25892, t25898, t25905, t4314, t6666, t6670, t6841, t7541, t81483, t86740, t86775, t86835, t87975);
        let t90001 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2172(t25365, t83555, t1530, t3231, t1649, t2749, t23788, t57893, t2752, t13487, t1877, t22959, t23286, t23290, t23295, t23796, t2522, t25901, t25921, t25930, t25934, t25938, t47645, t6666, t6670, t7541, t7650, t7656, t81483, t81525);
        let (t90003, t90020, t90022, t90026) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2173(t89880, t89920, t89957, t90001, t23858, t7685, t22607, t7688, t1390, t16018, t1983, t6878);
        let (t90029, t90034, t90036, t90038, t90040) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2174(t22574, t56194, t8643, t12461, t6995, t26161, t26163, t22581, t7685, t24987, t7000, t25985, t6876);
    (t90003, t90020, t90022, t90026, t90029, t90034, t90036, t90038, t90040)
}
