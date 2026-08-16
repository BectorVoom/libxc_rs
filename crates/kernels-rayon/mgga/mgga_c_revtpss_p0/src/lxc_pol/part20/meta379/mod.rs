//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1374;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1375;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1376;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1377;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta379(t853: f64, t9794: f64, t775: f64, t837: f64, t10760: f64, t10292: f64, t66: f64, t240: f64, t10688: f64, t243: f64, t268: f64, t2694: f64, t9784: f64, t10489: f64, t236: f64, t807: f64, t854: f64, t10681: f64, t2689: f64, t16: f64, t2236: f64, t281: f64, t39644: f64, t2645: f64, t10779: f64, t10786: f64, t14931: f64, t40583: f64, t10871: f64, t10773: f64, t10811: f64, t10764: f64, t10770: f64, t10771: f64, t125: f64, t14791: f64, t14894: f64, t2646: f64, t2745: f64, t2747: f64, t2754: f64, t40446: f64, t40600: f64, t40607: f64, t40611: f64, t40625: f64, t10696: f64, t72: f64, t245: f64, t10729: f64, t9775: f64, t10705: f64, t10716: f64, t10697: f64, t136: f64, t10627: f64, t221: f64, t2674: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40628, t40630, t40633, t40634, t40638, t40639) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1374(t853, t9794, t775, t837, t10760, t10292, t66, t240, t10688, t243, t268, t2694, t9784);
        let (t40643, t40645, t40649, t40650, t40654) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1375(t10489, t236, t807, t854, t10681, t2689, t16, t2236, t240, t243, t281, t39644);
        let (t40655, t40671) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1376(t2645, t775, t10779, t10786, t14931, t40583, t10871, t10773, t10811, t10489, t10764, t10770, t10771, t125, t14791, t14894, t2646, t2745, t2747, t2754, t40446, t40600, t40607, t40611, t40625, t40630, t40638, t40639, t40643, t40645, t40654, t837);
        let (t40673, t40679, t40681, t40686) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1377(t10696, t72, t245, t10729, t9775, t10705, t10716, t10697, t136, t10627, t221, t2674);
    (t40628, t40633, t40634, t40649, t40650, t40655, t40671, t40673, t40679, t40681, t40686)
}
