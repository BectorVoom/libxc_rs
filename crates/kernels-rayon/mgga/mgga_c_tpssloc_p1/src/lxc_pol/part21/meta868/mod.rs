//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta868 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3176;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3177;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3178;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3179;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3180;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3181;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta868(t11738: f64, t15560: f64, t15564: f64, t15612: f64, t15617: f64, t18300: f64, t19077: f64, t3252: f64, t3494: f64, t3509: f64, t3516: f64, t3577: f64, t3578: f64, t44836: f64, t44965: f64, t45037: f64, t4582: f64, t4980: f64, t4984: f64, t5005: f64, t5024: f64, t52621: f64, t52628: f64, t52649: f64, t52653: f64, t52664: f64, t52903: f64, t53372: f64, t53399: f64, t6219: f64, t15643: f64, t19201: f64, t3576: f64, t44951: f64, t6191: f64, t11668: f64, t15569: f64, t15663: f64, t15704: f64, t15708: f64, t15750: f64, t18210: f64, t18231: f64, t19056: f64, t3515: f64, t3580: f64, t44847: f64, t52666: f64, t52674: f64, t52680: f64, t52682: f64, t52684: f64, t52766: f64, t52879: f64, t13969: f64, t19061: f64, t11665: f64, t11678: f64, t11692: f64, t1227: f64, t14731: f64, t14736: f64, t14740: f64, t15654: f64, t1735: f64, t19016: f64, t19068: f64, t3490: f64, t4724: f64, t4987: f64, t5012: f64, t52725: f64, t52731: f64, t52733: f64, t55662: f64, t55666: f64, t5979: f64, t62044: f64, t15568: f64, t5064: f64, t1174: f64, t18206: f64, t44562: f64, t18958: f64, t15591: f64, t15714: f64, t18342: f64, t18387: f64, t44621: f64, t4950: f64, t5014: f64, t52751: f64, t52758: f64, t52773: f64, t53322: f64, t5971: f64, t5975: f64, t63420: f64, t248: f64, t45046: f64, t15438: f64, t15453: f64, t15527: f64, t15555: f64, t15637: f64, t15737: f64, t19080: f64, t3496: f64, t44886: f64, t44890: f64, t44894: f64, t5002: f64, t52776: f64, t52781: f64, t52792: f64, t52795: f64, t52801: f64, t1009: f64, t18571: f64, t1011: f64, t1212: f64, t3032: f64, t65253: f64, t3505: f64, t3514: f64, t1218: f64, t15455: f64, t15541: f64, t15545: f64, t15656: f64, t18590: f64, t18594: f64, t18955: f64, t19047: f64, t3511: f64, t3518: f64, t4972: f64, t52817: f64, t52845: f64, t52859: f64, t61798: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t65802 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3176(t11738, t15560, t15564, t15612, t15617, t18300, t19077, t3252, t3494, t3509, t3516, t3577, t3578, t44836, t44965, t45037, t4582, t4980, t4984, t5005, t5024, t52621, t52628, t52649, t52653, t52664, t52903, t53372, t53399, t6219);
        let t65835 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3177(t15643, t5024, t19201, t3576, t3577, t44951, t6191, t11668, t15569, t15663, t15704, t15708, t15750, t18210, t18231, t19056, t3494, t3515, t3580, t44847, t4582, t52666, t52674, t52680, t52682, t52684, t52766, t52879);
        let t65883 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3178(t13969, t19061, t3515, t11665, t11668, t11678, t11692, t1227, t14731, t14736, t14740, t15654, t1735, t19016, t19068, t3490, t3509, t3516, t3577, t3578, t4582, t4724, t4987, t5012, t52725, t52731, t52733, t55662, t55666, t5979, t62044);
        let t65925 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3179(t15568, t5064, t1174, t18206, t44562, t1227, t13969, t18958, t11665, t11668, t11678, t11692, t15569, t15591, t15714, t18342, t18387, t3490, t3494, t3509, t3516, t3577, t3578, t3580, t44621, t4950, t5014, t52751, t52758, t52773, t53322, t5971, t5975, t63420);
        let t65954 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3180(t1227, t248, t45046, t5971, t15643, t5005, t15438, t15453, t15527, t15555, t15637, t15737, t19080, t3496, t44886, t44890, t44894, t4582, t5002, t52776, t52781, t52792, t52795, t52801, t62044);
        let (t65955, t65990) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3181(t1009, t18571, t1011, t1212, t3032, t65253, t3505, t3514, t1218, t1227, t15455, t15541, t15545, t15656, t18590, t18594, t18955, t19047, t3490, t3496, t3511, t3518, t4582, t4972, t5005, t52817, t52845, t52859, t61798);
    (t65802, t65835, t65883, t65925, t65954, t65955, t65990)
}
