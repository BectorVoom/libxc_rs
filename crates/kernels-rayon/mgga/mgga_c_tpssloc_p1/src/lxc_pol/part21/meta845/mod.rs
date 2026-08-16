//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta845 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3056;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3057;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3058;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3059;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta845(t2244: f64, t43763: f64, t5392: f64, t123: f64, t43809: f64, t63380: f64, t63382: f64, t63384: f64, t63388: f64, t63392: f64, t63396: f64, t63398: f64, t63400: f64, t63404: f64, t63408: f64, t63412: f64, t63417: f64, t3265: f64, t3313: f64, t6021: f64, t11190: f64, t5989: f64, t14850: f64, t14937: f64, t3375: f64, t6063: f64, t1136: f64, t11365: f64, t14829: f64, t15153: f64, t15165: f64, t15179: f64, t15219: f64, t1695: f64, t18615: f64, t18622: f64, t3376: f64, t3377: f64, t3378: f64, t3395: f64, t3401: f64, t3403: f64, t436: f64, t44155: f64, t51382: f64, t51389: f64, t51392: f64, t51486: f64, t51521: f64, t51727: f64, t6085: f64, t6088: f64, t63280: f64, t63283: f64, t63290: f64, t63325: f64, t63346: f64, t63376: f64, t18893: f64, t3359: f64, t11303: f64, t11350: f64, t11415: f64, t11420: f64, t15117: f64, t15146: f64, t15159: f64, t15168: f64, t15172: f64, t1683: f64, t18631: f64, t18634: f64, t18637: f64, t18640: f64, t18643: f64, t18644: f64, t18650: f64, t18894: f64, t3332: f64, t3333: f64, t3351: f64, t3357: f64, t44214: f64, t44361: f64, t4824: f64, t51427: f64, t51599: f64, t51604: f64, t6037: f64, t6053: f64, t6056: f64, t11285: f64, t6084: f64, t18785: f64, t3307: f64, t11275: f64, t6024: f64, t11310: f64, t11361: f64, t1155: f64, t1156: f64, t15126: f64, t15136: f64, t15182: f64, t15185: f64, t15210: f64, t15222: f64, t15226: f64, t18619: f64, t18623: f64, t43692: f64, t44220: f64, t44223: f64, t4861: f64, t51376: f64, t51680: f64, t6068: f64, t6069: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63420, t63422) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3056(t2244, t43763, t5392, t123, t43809);
        let t63424 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3057(t63380, t63382, t63384, t63388, t63392, t63396, t63398, t63400, t63404, t63408, t63412, t63417, t63422);
        let (t63446, t63449, t63451, t63457) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3058(t3265, t3313, t6021, t11190, t5989, t14850, t14937, t3375, t6063, t1136, t11365, t14829, t15153, t15165, t15179, t15219, t1695, t18615, t18622, t3376, t3377, t3378, t3395, t3401, t3403, t436, t44155, t51382, t51389, t51392, t51486, t51521, t51727, t6085, t6088, t63280, t63283, t63290, t63325, t63346, t63376, t63424);
        let t63506 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3059(t18893, t3359, t11303, t11350, t1136, t11415, t11420, t15117, t15146, t15159, t15165, t15168, t15172, t1683, t18631, t18634, t18637, t18640, t18643, t18644, t18650, t18894, t3332, t3333, t3351, t3357, t44214, t44361, t4824, t51427, t51599, t51604, t6037, t6053, t6056);
        let (t63557, t63560, t63561) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3060(t11285, t6084, t18785, t3403, t3307, t3313, t5989, t11275, t3265, t6024, t11310, t11361, t1155, t1156, t14829, t15126, t15136, t15182, t15185, t15210, t15222, t15226, t18615, t18619, t18622, t18623, t18643, t3351, t3357, t3376, t3377, t3395, t3401, t43692, t44220, t44223, t4861, t51376, t51680, t6068, t6069, t6088, t63283);
    (t63420, t63422, t63446, t63449, t63451, t63457, t63506, t63557, t63560, t63561)
}
