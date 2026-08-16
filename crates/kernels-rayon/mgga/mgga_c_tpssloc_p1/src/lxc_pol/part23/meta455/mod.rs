//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta455 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1313;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1314;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1315;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1316;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1317;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1318;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta455(t59028: f64, t145: f64, t185: f64, t75929: f64, t39658: f64, t41258: f64, t41262: f64, t76024: f64, t76025: f64, t76026: f64, t76027: f64, t76030: f64, t76031: f64, t76034: f64, t5527: f64, t5544: f64, t1504: f64, t1506: f64, t16729: f64, t16736: f64, t20800: f64, t20835: f64, t20843: f64, t20846: f64, t20849: f64, t225: f64, t228: f64, t230: f64, t2671: f64, t41315: f64, t4225: f64, t4226: f64, t5601: f64, t5605: f64, t5608: f64, t75978: f64, t76006: f64, t76007: f64, t76009: f64, t76010: f64, t76013: f64, t76014: f64, t76021: f64, t824: f64, t232: f64, t5584: f64, t40933: f64, t9975: f64, t13251: f64, t13262: f64, t1484: f64, t16839: f64, t16891: f64, t20885: f64, t20887: f64, t20972: f64, t2632: f64, t2643: f64, t2645: f64, t4178: f64, t4180: f64, t5591: f64, t5617: f64, t67607: f64, t67612: f64, t67625: f64, t67637: f64, t67639: f64, t68246: f64, t9646: f64, t119: f64, t16836: f64, t20974: f64, t20978: f64, t20986: f64, t20988: f64, t210: f64, t2571: f64, t2701: f64, t41161: f64, t46546: f64, t58421: f64, t67620: f64, t67660: f64, t67675: f64, t820: f64, t843: f64, t1516: f64, t16976: f64, t20896: f64, t20908: f64, t40971: f64, t4172: f64, t46577: f64, t5624: f64, t5628: f64, t58550: f64, t67690: f64, t67692: f64, t67729: f64, t67735: f64, t68203: f64, t847: f64, t1510: f64, t20756: f64, t20852: f64, t20882: f64, t20891: f64, t20983: f64, t41467: f64, t4181: f64, t5587: f64, t5593: f64, t58574: f64, t58576: f64, t58642: f64, t58811: f64, t67852: f64, t67854: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76035, t76037, t76038) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1313(t59028, t145, t185, t75929, t39658, t41258, t41262, t76024, t76025, t76026, t76027, t76030, t76031, t76034);
        let (t76056, t76063, t76073) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1314(t5527, t5544, t1504, t1506, t16729, t16736, t20800, t20835, t20843, t20846, t20849, t225, t228, t230, t2671, t41315, t4225, t4226, t5601, t5605, t5608, t75978, t76006, t76007, t76009, t76010, t76013, t76014, t76021, t76038, t824);
        let (t76074, t76085, t76086, t76090, t76132) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1315(t232, t76073, t5584, t40933, t9975, t13251, t13262, t1484, t16839, t16891, t20885, t20887, t20972, t2632, t2643, t2645, t4178, t4180, t5527, t5591, t5617, t67607, t67612, t67625, t67637, t67639, t68246, t9646);
        let t76167 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1316(t119, t13251, t16836, t16839, t20885, t20974, t20978, t20986, t20988, t210, t2571, t2643, t2645, t2701, t41161, t4178, t4180, t46546, t5591, t58421, t67620, t67660, t67675, t76056, t76063, t820, t843);
        let t76193 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1317(t1516, t16976, t20896, t20908, t40971, t4172, t46577, t5624, t5628, t58550, t67690, t67692, t67729, t67735, t68203, t75978, t76056, t820, t843, t847);
        let t76227 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1318(t13251, t1510, t16836, t16839, t20756, t20852, t20882, t20891, t20983, t232, t2632, t2643, t2645, t41467, t4178, t4180, t4181, t5544, t5587, t5593, t58574, t58576, t58642, t58811, t67620, t67852, t67854);
    (t76035, t76037, t76056, t76063, t76074, t76085, t76086, t76090, t76132, t76167, t76193, t76227)
}
