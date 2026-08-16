//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1899;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1900;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1901;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1902;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1903;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta418(t13458: f64, t665: f64, t2366: f64, t4263: f64, t10227: f64, t1504: f64, t2350: f64, t2349: f64, t97: f64, t2255: f64, t658: f64, t2256: f64, t4269: f64, t100: f64, t580: f64, t22: f64, t4273: f64, t10241: f64, t1509: f64, t2358: f64, t105: f64, t2357: f64, t661: f64, t2362: f64, t4279: f64, t108: f64, t4283: f64, t1505: f64, t1507: f64, t2344: f64, t2359: f64, t2363: f64, t4270: f64, t4274: f64, t656: f64, t655: f64, t10201: f64, t10202: f64, t10204: f64, t10206: f64, t13448: f64, t13451: f64, t13453: f64, t13455: f64, t69: f64, t114: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13459, t13462, t13472, t13475, t13476, t13479) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1899(t13458, t665, t2366, t4263, t10227, t1504, t2350, t2349, t97, t2255, t658, t2256, t4269);
        let (t13482, t13485, t13493, t13496, t13497, t13500) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1900(t100, t580, t22, t4273, t10241, t1509, t2358, t105, t2357, t2255, t661, t2362, t4279);
        let t13509 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1901(t108, t580, t22, t4283, t105, t13472, t13475, t13476, t13479, t13482, t13485, t13493, t13496, t13497, t13500, t1505, t1507, t2344, t2359, t2363, t4270, t4274, t656, t97);
        let (t13510, t13513) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1902(t13509, t655, t10201, t10202, t10204, t10206, t13448, t13451, t13453, t13455, t13459, t13462, t69);
        let t13514 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1903(t114, t13513);
    (t13459, t13462, t13472, t13475, t13476, t13479, t13482, t13485, t13496, t13509, t13510, t13514)
}
