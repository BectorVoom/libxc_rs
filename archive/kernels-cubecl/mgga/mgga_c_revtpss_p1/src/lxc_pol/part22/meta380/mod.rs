//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta380 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1938;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1939;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1940;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1941;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1942;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta380<F: Float>(t100: F, t580: F, t22: F, t4273: F, t10241: F, t1509: F, t2358: F, t105: F, t2357: F, t2255: F, t661: F, t2362: F, t4279: F, t108: F, t4283: F, t13472: F, t13475: F, t13476: F, t13479: F, t1505: F, t1507: F, t2344: F, t2359: F, t2363: F, t4270: F, t4274: F, t656: F, t97: F, t655: F, t10201: F, t10202: F, t10204: F, t10206: F, t13448: F, t13451: F, t13453: F, t13455: F, t13459: F, t13462: F, t69: F, t114: F, t10416: F, t1312: F, t13425: F, t13426: F, t13429: F, t13435: F, t13440: F, t1518: F, t2322: F, t2371: F, t4248: F, t4292: F, t5523: F, t670: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13482, t13485, t13493, t13496, t13497, t13500) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1938::<F>(t100, t580, t22, t4273, t10241, t1509, t2358, t105, t2357, t2255, t661, t2362, t4279);
        let (t13503, t13506, t13509) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1939::<F>(t108, t580, t22, t4283, t105, t13472, t13475, t13476, t13479, t13482, t13485, t13493, t13496, t13497, t13500, t1505, t1507, t2344, t2359, t2363, t4270, t4274, t656, t97);
        let (t13510, t13513) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1940::<F>(t13509, t655, t10201, t10202, t10204, t10206, t13448, t13451, t13453, t13455, t13459, t13462, t69);
        let t13514 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1941::<F>(t114, t13513);
        let t13517 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1942::<F>(t10416, t1312, t13425, t13426, t13429, t13435, t13440, t13514, t1518, t2322, t2371, t4248, t4292, t5523, t670);
    (t13493, t13496, t13497, t13500, t13503, t13506, t13509, t13510, t13514, t13517)
}
