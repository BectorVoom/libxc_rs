//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta375 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1544;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1545;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1546;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta375(t13546: f64, t977: f64, t13555: f64, t2979: f64, t13528: f64, t13532: f64, t10214: f64, t13537: f64, t13969: f64, t4595: f64, t3130: f64, t1616: f64, t2780: f64, t3071: f64, t2771: f64, t10408: f64, t1539: f64, t3121: f64, t3048: f64, t4571: f64, t10390: f64, t10891: f64, t10904: f64, t10937: f64, t10957: f64, t1622: f64, t3070: f64, t3098: f64, t4575: f64, t4596: f64, t4600: f64, t4644: f64, t973: f64, t3109: f64, t4630: f64, t4650: f64, t884: f64, t10436: f64, t10441: f64, t10449: f64, t10455: f64, t10460: f64, t10490: f64, t10496: f64, t10504: f64, t10511: f64, t10517: f64, t10863: f64, t10866: f64, t10871: f64, t1618: f64, t4636: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t14006, t14009, t14012, t14015, t14018, t14025, t14027, t14032) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1544(t13546, t977, t13555, t2979, t13528, t13532, t10214, t13537, t13969, t4595, t3130, t1616, t2780);
        let (t14033, t14037, t14041, t14050) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1545(t14032, t3071, t1616, t2771, t10408, t1539, t3121, t3048, t4571, t10390, t10891, t10904, t10937, t10957, t14006, t14009, t14012, t14015, t14018, t14027, t1622, t3070, t3098, t4575, t4596, t4600, t4644, t973);
        let (t14069, t14074) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1546(t3109, t4630, t4650, t884, t3071, t10436, t10441, t10449, t10455, t10460, t10490, t10496, t10504, t10511, t10517, t10863, t10866, t10871, t1618, t1622, t3048, t3070, t4636);
    (t14025, t14033, t14037, t14041, t14050, t14069, t14074)
}
