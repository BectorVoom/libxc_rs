//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1415;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1416;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1417;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1418;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1419;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1420;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1421;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta376<F: Float>(t100: F, t580: F, t22: F, t4273: F, t10241: F, t1509: F, t2358: F, t105: F, t2357: F, t2255: F, t661: F, t2362: F, t4279: F, t108: F, t4283: F, t13472: F, t13475: F, t13476: F, t13479: F, t1505: F, t1507: F, t2344: F, t2359: F, t2363: F, t4270: F, t4274: F, t656: F, t97: F, t655: F, t10201: F, t10202: F, t10204: F, t10206: F, t13448: F, t13451: F, t13453: F, t13455: F, t13459: F, t13462: F, t69: F, t114: F, t10416: F, t1312: F, t13425: F, t13426: F, t13429: F, t13435: F, t13440: F, t1518: F, t2322: F, t2371: F, t4248: F, t4292: F, t5523: F, t670: F, t1843: F, t1310: F, t3813: F, t5517: F, t508: F, t1453: F, t1502: F, t1519: F, t2328: F, t2372: F, t4254: F, t4257: F, t4293: F, t4297: F, t5528: F, t569: F, t651: F, t30: F, t1468: F, t9335: F, t2: F, t3833: F, t605: F, t2257: F, t3834: F, t513: F, t5549: F, t5552: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13482, t13485, t13493, t13496, t13497, t13500) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1415::<F>(t100, t580, t22, t4273, t10241, t1509, t2358, t105, t2357, t2255, t661, t2362, t4279);
        let t13509 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1416::<F>(t108, t580, t22, t4283, t105, t13472, t13475, t13476, t13479, t13482, t13485, t13493, t13496, t13497, t13500, t1505, t1507, t2344, t2359, t2363, t4270, t4274, t656, t97);
        let t13513 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1417::<F>(t13509, t655, t10201, t10202, t10204, t10206, t13448, t13451, t13453, t13455, t13459, t13462, t69);
        let t13514 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1418::<F>(t114, t13513);
        let t13517 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1419::<F>(t10416, t1312, t13425, t13426, t13429, t13435, t13440, t13514, t1518, t2322, t2371, t4248, t4292, t5523, t670);
        let (t13521, t13532, t13537, t13540, t13544, t13547) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1420::<F>(t1843, t2371, t1310, t4292, t1518, t3813, t5517, t670, t13514, t508, t10416, t13435, t13517, t1453, t1502, t1519, t2322, t2328, t2372, t4248, t4254, t4257, t4293, t4297, t5528, t569, t651);
        let (t13554, t13564) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1421::<F>(t30, t1468, t9335, t2, t3833, t580, t605, t22, t2257, t3834, t513, t5549, t5552, zeta_threshold);
    (t13509, t13514, t13517, t13521, t13532, t13537, t13540, t13544, t13547, t13554, t13564)
}
