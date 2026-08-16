//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta304 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1037;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1038;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta304(t1615: f64, t5914: f64, t1060: f64, t21594: f64, t381: f64, t21390: f64, t11048: f64, t1625: f64, t5872: f64, t3188: f64, t11060: f64, t11066: f64, t3201: f64, t5866: f64, t1629: f64, t1058: f64, t11046: f64, t11059: f64, t11065: f64, t14608: f64, t14618: f64, t1610: f64, t1630: f64, t1632: f64, t18086: f64, t21481: f64, t21615: f64, t21618: f64, t21623: f64, t3186: f64, t3200: f64, t353: f64, t384: f64, t4669: f64, t5903: f64, t5929: f64, t5933: f64, t5937: f64, t5939: f64, t5941: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21627, t21634, t21635, t21638, t21643, t21644, t21647, t21650) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1037(t1615, t5914, t1060, t21594, t381, t21390, t11048, t1625, t5872, t3188, t11060, t11066);
        let (t21653, t21657, t21662) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1038(t21643, t3201, t3188, t5866, t1629, t1058, t11046, t11059, t11065, t14608, t14618, t1610, t1630, t1632, t18086, t21481, t21615, t21618, t21623, t21627, t21635, t21638, t21644, t21647, t21650, t3186, t3200, t353, t384, t4669, t5903, t5929, t5933, t5937, t5939, t5941);
    (t21627, t21634, t21635, t21638, t21644, t21647, t21650, t21653, t21657, t21662)
}
