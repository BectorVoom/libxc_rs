//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1415;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1416;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1417;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1418;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1419;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1420;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1421;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta376(t100: f64, t580: f64, t22: f64, t4273: f64, t10241: f64, t1509: f64, t2358: f64, t105: f64, t2357: f64, t2255: f64, t661: f64, t2362: f64, t4279: f64, t108: f64, t4283: f64, t13472: f64, t13475: f64, t13476: f64, t13479: f64, t1505: f64, t1507: f64, t2344: f64, t2359: f64, t2363: f64, t4270: f64, t4274: f64, t656: f64, t97: f64, t655: f64, t10201: f64, t10202: f64, t10204: f64, t10206: f64, t13448: f64, t13451: f64, t13453: f64, t13455: f64, t13459: f64, t13462: f64, t69: f64, t114: f64, t10416: f64, t1312: f64, t13425: f64, t13426: f64, t13429: f64, t13435: f64, t13440: f64, t1518: f64, t2322: f64, t2371: f64, t4248: f64, t4292: f64, t5523: f64, t670: f64, t1843: f64, t1310: f64, t3813: f64, t5517: f64, t508: f64, t1453: f64, t1502: f64, t1519: f64, t2328: f64, t2372: f64, t4254: f64, t4257: f64, t4293: f64, t4297: f64, t5528: f64, t569: f64, t651: f64, t30: f64, t1468: f64, t9335: f64, t2: f64, t3833: f64, t605: f64, t2257: f64, t3834: f64, t513: f64, t5549: f64, t5552: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13482, t13485, t13493, t13496, t13497, t13500) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1415(t100, t580, t22, t4273, t10241, t1509, t2358, t105, t2357, t2255, t661, t2362, t4279);
        let t13509 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1416(t108, t580, t22, t4283, t105, t13472, t13475, t13476, t13479, t13482, t13485, t13493, t13496, t13497, t13500, t1505, t1507, t2344, t2359, t2363, t4270, t4274, t656, t97);
        let t13513 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1417(t13509, t655, t10201, t10202, t10204, t10206, t13448, t13451, t13453, t13455, t13459, t13462, t69);
        let t13514 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1418(t114, t13513);
        let t13517 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1419(t10416, t1312, t13425, t13426, t13429, t13435, t13440, t13514, t1518, t2322, t2371, t4248, t4292, t5523, t670);
        let (t13521, t13532, t13537, t13540, t13544, t13547) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1420(t1843, t2371, t1310, t4292, t1518, t3813, t5517, t670, t13514, t508, t10416, t13435, t13517, t1453, t1502, t1519, t2322, t2328, t2372, t4248, t4254, t4257, t4293, t4297, t5528, t569, t651);
        let (t13554, t13564) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1421(t30, t1468, t9335, t2, t3833, t580, t605, t22, t2257, t3834, t513, t5549, t5552, zeta_threshold);
    (t13509, t13514, t13517, t13521, t13532, t13537, t13540, t13544, t13547, t13554, t13564)
}
