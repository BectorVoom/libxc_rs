//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1374;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1375;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1376;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1377;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta379<F: Float>(t853: F, t9794: F, t775: F, t837: F, t10760: F, t10292: F, t66: F, t240: F, t10688: F, t243: F, t268: F, t2694: F, t9784: F, t10489: F, t236: F, t807: F, t854: F, t10681: F, t2689: F, t16: F, t2236: F, t281: F, t39644: F, t2645: F, t10779: F, t10786: F, t14931: F, t40583: F, t10871: F, t10773: F, t10811: F, t10764: F, t10770: F, t10771: F, t125: F, t14791: F, t14894: F, t2646: F, t2745: F, t2747: F, t2754: F, t40446: F, t40600: F, t40607: F, t40611: F, t40625: F, t10696: F, t72: F, t245: F, t10729: F, t9775: F, t10705: F, t10716: F, t10697: F, t136: F, t10627: F, t221: F, t2674: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40628, t40630, t40633, t40634, t40638, t40639) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1374::<F>(t853, t9794, t775, t837, t10760, t10292, t66, t240, t10688, t243, t268, t2694, t9784);
        let (t40643, t40645, t40649, t40650, t40654) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1375::<F>(t10489, t236, t807, t854, t10681, t2689, t16, t2236, t240, t243, t281, t39644);
        let (t40655, t40671) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1376::<F>(t2645, t775, t10779, t10786, t14931, t40583, t10871, t10773, t10811, t10489, t10764, t10770, t10771, t125, t14791, t14894, t2646, t2745, t2747, t2754, t40446, t40600, t40607, t40611, t40625, t40630, t40638, t40639, t40643, t40645, t40654, t837);
        let (t40673, t40679, t40681, t40686) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1377::<F>(t10696, t72, t245, t10729, t9775, t10705, t10716, t10697, t136, t10627, t221, t2674);
    (t40628, t40633, t40634, t40649, t40650, t40655, t40671, t40673, t40679, t40681, t40686)
}
