//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta426 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1256;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1257;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta426(t1036: f64, t21483: f64, t1041: f64, t13969: f64, t21511: f64, t10413: f64, t10422: f64, t21531: f64, t21486: f64, t3130: f64, t21565: f64, t3070: f64, t21126: f64, t2970: f64, t973: f64, t21569: f64, t42488: f64, t10231: f64, t21122: f64, t21689: f64, t225: f64, t21669: f64, t21684: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t70766, t70792, t70800, t70805, t70846) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1256(t1036, t21483, t1041, t13969, t21511, t10413, t10422, t21531, t21486, t3130, t21565, t3070);
        let (t70867, t70912, t70929, t70978, t70980, t70987) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1257(t21126, t2970, t973, t21569, t3070, t42488, t10231, t21122, t21689, t225, t21669, t21684);
    (t70766, t70792, t70800, t70805, t70846, t70867, t70912, t70929, t70978, t70980, t70987)
}
