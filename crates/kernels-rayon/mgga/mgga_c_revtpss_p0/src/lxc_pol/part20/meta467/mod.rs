//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta467 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1786;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1787;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1788;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1789;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1790;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1791;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1792;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1793;
use chunk8::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1794;
use chunk9::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1795;
use chunk10::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1796;
use chunk11::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1797;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta467(t556: f64, t786: f64, t9656: f64, t686: f64, t72: f64, t9658: f64, t10150: f64, t2435: f64, t9651: f64, t9680: f64, t1358: f64, t2439: f64, t4066: f64, t785: f64, t9303: f64, t9641: f64, t9635: f64, t213: f64, t225: f64, t4071: f64, t47343: f64, t47568: f64, t47570: f64, t47574: f64, t47580: f64, t47591: f64, t47593: f64, t47595: f64, t47601: f64, t561: f64, t9652: f64, t1448: f64, t1450: f64, t198: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t46961: f64, t46963: f64, t46965: f64, t46968: f64, t46970: f64, t46972: f64, t46974: f64, t46976: f64, t46978: f64, t47468: f64, t47518: f64, t47566: f64, t532: f64, t9400: f64, t39747: f64, t39750: f64, t39756: f64, t39760: f64, t46980: f64, t46982: f64, t46984: f64, t46988: f64, t46990: f64, t46992: f64, t46994: f64, t46996: f64, t46998: f64, t47000: f64, t47003: f64, t9590: f64, t9593: f64, t1353: f64, t13625: f64, t25802: f64, t3829: f64, t3889: f64, t39773: f64, t39783: f64, t39786: f64, t39791: f64, t39795: f64, t4139: f64, t47006: f64, t47008: f64, t47010: f64, t47012: f64, t47014: f64, t47017: f64, t5536: f64, t9599: f64, t10179: f64, t39799: f64, t39807: f64, t39813: f64, t47020: f64, t47057: f64, t47059: f64, t47061: f64, t47064: f64, t47067: f64, t47070: f64, t47072: f64, t47074: f64, t566: f64, t9547: f64, t9628: f64, t4144: f64, t4146: f64, t25177: f64, t39989: f64, t4135: f64, t4140: f64, t47076: f64, t47079: f64, t47082: f64, t47084: f64, t47086: f64, t47088: f64, t47090: f64, t47092: f64, t47094: f64, t47096: f64, t47098: f64, t5541: f64, t40067: f64, t40072: f64, t47100: f64, t47102: f64, t47107: f64, t47109: f64, t47111: f64, t47114: f64, t47116: f64, t47118: f64, t47120: f64, t47122: f64, t47124: f64, t47126: f64, t9984: f64, t40076: f64, t40079: f64, t4147: f64, t47128: f64, t47131: f64, t47134: f64, t47136: f64, t47138: f64, t47140: f64, t47142: f64, t47144: f64, t47146: f64, t47148: f64, t47150: f64, t47152: f64, t10192: f64, t10194: f64, t10415: f64, t10416: f64, t1310: f64, t1315: f64, t13207: f64, t13435: f64, t2320: f64, t2328: f64, t2372: f64, t3813: f64, t3821: f64, t4151: f64, t46126: f64, t46129: f64, t46137: f64, t46233: f64, t46349: f64, t508: f64, t511: f64, t649: f64, t651: f64, t671: f64, t94: f64, t46250: f64, t10259: f64, t116: f64, t117: f64, t13232: f64, t13240: f64, t13243: f64, t13244: f64, t13247: f64, t1459: f64, t1461: f64, t2327: f64, t2371: f64, t4158: f64, t4162: f64, t4165: f64, t572: f64, t573: f64, param_d: f64, t13226: f64, t13250: f64, t1456: f64, t1458: f64, t1464: f64, t3: f64, t39397: f64, t39399: f64, t39401: f64, t39403: f64, t4154: f64, t4168: f64, t575: f64) -> f64 {
        let (t47606, t47608, t47612, t47616) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1786(t556, t786, t9656, t686, t72, t9658, t10150, t2435, t9651, t9680, t1358, t2439, t4066, t785);
        let t47622 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1787(t9303, t9641, t2435, t9635, t213, t225, t4071, t47343, t47568, t47570, t47574, t47580, t47591, t47593, t47595, t47601, t47606, t47608, t47612, t47616, t561, t9652);
        let t47632 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1788(t1448, t1450, t198, t39483, t39520, t39528, t39531, t46961, t46963, t46965, t46968, t46970, t46972, t46974, t46976, t46978, t47468, t47518, t47566, t47622, t532, t9400);
        let t47634 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1789(t39747, t39750, t39756, t39760, t46980, t46982, t46984, t46988, t46990, t46992, t46994, t46996, t46998, t47000, t47003);
        let t47648 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1790(t9590, t9593, t1353, t13625, t25802, t3829, t3889, t39773, t39783, t39786, t39791, t39795, t4139, t47006, t47008, t47010, t47012, t47014, t47017, t5536, t9599);
        let t47662 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1791(t10179, t1450, t1353, t3889, t39799, t39807, t39813, t4139, t47020, t47057, t47059, t47061, t47064, t47067, t47070, t47072, t47074, t5536, t566, t9547, t9628);
        let t47676 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1792(t4144, t4146, t198, t25177, t39989, t4135, t4139, t4140, t47076, t47079, t47082, t47084, t47086, t47088, t47090, t47092, t47094, t47096, t47098, t532, t5541, t9628);
        let t47681 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1793(t40067, t40072, t4140, t47100, t47102, t47107, t47109, t47111, t47114, t47116, t47118, t47120, t47122, t47124, t47126, t5536, t9984);
        let t47687 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1794(t4135, t198, t40076, t40079, t4147, t47128, t47131, t47134, t47136, t47138, t47140, t47142, t47144, t47146, t47148, t47150, t47152, t532);
        let t47692 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1795(t10192, t10194, t10415, t10416, t1310, t1315, t13207, t13435, t2320, t2328, t2372, t3813, t3821, t4151, t46126, t46129, t46137, t46233, t46349, t47632, t47634, t47648, t47662, t47676, t47681, t47687, t508, t511, t649, t651, t671, t94);
        let (t47693, t47728) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1796(t46250, t47692, t10259, t116, t117, t13232, t13240, t13243, t13244, t13247, t1459, t1461, t2327, t2371, t4158, t4162, t4165, t46137, t46233, t572, t573, param_d);
        let tv4rho40 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1797(t13226, t13250, t1456, t1458, t1464, t3, t39397, t39399, t39401, t39403, t4154, t4168, t47693, t47728, t575);
    tv4rho40
}
