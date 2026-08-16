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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta467<F: Float>(t556: F, t786: F, t9656: F, t686: F, t72: F, t9658: F, t10150: F, t2435: F, t9651: F, t9680: F, t1358: F, t2439: F, t4066: F, t785: F, t9303: F, t9641: F, t9635: F, t213: F, t225: F, t4071: F, t47343: F, t47568: F, t47570: F, t47574: F, t47580: F, t47591: F, t47593: F, t47595: F, t47601: F, t561: F, t9652: F, t1448: F, t1450: F, t198: F, t39483: F, t39520: F, t39528: F, t39531: F, t46961: F, t46963: F, t46965: F, t46968: F, t46970: F, t46972: F, t46974: F, t46976: F, t46978: F, t47468: F, t47518: F, t47566: F, t532: F, t9400: F, t39747: F, t39750: F, t39756: F, t39760: F, t46980: F, t46982: F, t46984: F, t46988: F, t46990: F, t46992: F, t46994: F, t46996: F, t46998: F, t47000: F, t47003: F, t9590: F, t9593: F, t1353: F, t13625: F, t25802: F, t3829: F, t3889: F, t39773: F, t39783: F, t39786: F, t39791: F, t39795: F, t4139: F, t47006: F, t47008: F, t47010: F, t47012: F, t47014: F, t47017: F, t5536: F, t9599: F, t10179: F, t39799: F, t39807: F, t39813: F, t47020: F, t47057: F, t47059: F, t47061: F, t47064: F, t47067: F, t47070: F, t47072: F, t47074: F, t566: F, t9547: F, t9628: F, t4144: F, t4146: F, t25177: F, t39989: F, t4135: F, t4140: F, t47076: F, t47079: F, t47082: F, t47084: F, t47086: F, t47088: F, t47090: F, t47092: F, t47094: F, t47096: F, t47098: F, t5541: F, t40067: F, t40072: F, t47100: F, t47102: F, t47107: F, t47109: F, t47111: F, t47114: F, t47116: F, t47118: F, t47120: F, t47122: F, t47124: F, t47126: F, t9984: F, t40076: F, t40079: F, t4147: F, t47128: F, t47131: F, t47134: F, t47136: F, t47138: F, t47140: F, t47142: F, t47144: F, t47146: F, t47148: F, t47150: F, t47152: F, t10192: F, t10194: F, t10415: F, t10416: F, t1310: F, t1315: F, t13207: F, t13435: F, t2320: F, t2328: F, t2372: F, t3813: F, t3821: F, t4151: F, t46126: F, t46129: F, t46137: F, t46233: F, t46349: F, t508: F, t511: F, t649: F, t651: F, t671: F, t94: F, t46250: F, t10259: F, t116: F, t117: F, t13232: F, t13240: F, t13243: F, t13244: F, t13247: F, t1459: F, t1461: F, t2327: F, t2371: F, t4158: F, t4162: F, t4165: F, t572: F, t573: F, param_d: F, t13226: F, t13250: F, t1456: F, t1458: F, t1464: F, t3: F, t39397: F, t39399: F, t39401: F, t39403: F, t4154: F, t4168: F, t575: F) -> F {
        let (t47606, t47608, t47612, t47616) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1786::<F>(t556, t786, t9656, t686, t72, t9658, t10150, t2435, t9651, t9680, t1358, t2439, t4066, t785);
        let t47622 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1787::<F>(t9303, t9641, t2435, t9635, t213, t225, t4071, t47343, t47568, t47570, t47574, t47580, t47591, t47593, t47595, t47601, t47606, t47608, t47612, t47616, t561, t9652);
        let t47632 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1788::<F>(t1448, t1450, t198, t39483, t39520, t39528, t39531, t46961, t46963, t46965, t46968, t46970, t46972, t46974, t46976, t46978, t47468, t47518, t47566, t47622, t532, t9400);
        let t47634 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1789::<F>(t39747, t39750, t39756, t39760, t46980, t46982, t46984, t46988, t46990, t46992, t46994, t46996, t46998, t47000, t47003);
        let t47648 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1790::<F>(t9590, t9593, t1353, t13625, t25802, t3829, t3889, t39773, t39783, t39786, t39791, t39795, t4139, t47006, t47008, t47010, t47012, t47014, t47017, t5536, t9599);
        let t47662 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1791::<F>(t10179, t1450, t1353, t3889, t39799, t39807, t39813, t4139, t47020, t47057, t47059, t47061, t47064, t47067, t47070, t47072, t47074, t5536, t566, t9547, t9628);
        let t47676 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1792::<F>(t4144, t4146, t198, t25177, t39989, t4135, t4139, t4140, t47076, t47079, t47082, t47084, t47086, t47088, t47090, t47092, t47094, t47096, t47098, t532, t5541, t9628);
        let t47681 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1793::<F>(t40067, t40072, t4140, t47100, t47102, t47107, t47109, t47111, t47114, t47116, t47118, t47120, t47122, t47124, t47126, t5536, t9984);
        let t47687 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1794::<F>(t4135, t198, t40076, t40079, t4147, t47128, t47131, t47134, t47136, t47138, t47140, t47142, t47144, t47146, t47148, t47150, t47152, t532);
        let t47692 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1795::<F>(t10192, t10194, t10415, t10416, t1310, t1315, t13207, t13435, t2320, t2328, t2372, t3813, t3821, t4151, t46126, t46129, t46137, t46233, t46349, t47632, t47634, t47648, t47662, t47676, t47681, t47687, t508, t511, t649, t651, t671, t94);
        let (t47693, t47728) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1796::<F>(t46250, t47692, t10259, t116, t117, t13232, t13240, t13243, t13244, t13247, t1459, t1461, t2327, t2371, t4158, t4162, t4165, t46137, t46233, t572, t573, param_d);
        let tv4rho40 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1797::<F>(t13226, t13250, t1456, t1458, t1464, t3, t39397, t39399, t39401, t39403, t4154, t4168, t47693, t47728, t575);
    tv4rho40
}
