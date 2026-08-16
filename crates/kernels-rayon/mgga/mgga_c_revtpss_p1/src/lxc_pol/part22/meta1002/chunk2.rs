//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3411/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3411(t11509: f64, t6205: f64, t19247: f64, t945: f64, t2967: f64, t6152: f64, t11461: f64, t11466: f64, t11507: f64, t15234: f64, t1634: f64, t19173: f64, t19303: f64, t19304: f64, t19310: f64, t2944: f64, t2963: f64, t2968: f64, t2971: f64, t2987: f64, t2988: f64, t3006: f64, t3012: f64, t41751: f64, t41759: f64, t4711: f64, t6174: f64, t6206: f64, t6209: f64, t63226: f64, t63228: f64, t63579: f64, t63581: f64, t955: f64) -> f64 {
    let t64043 = t6205 * t11509;
    let t64055 = t19247 * t945;
    let t64060 = t6152 * t2967;
    let t64068 = -0.23392894490538584828e1_f64 * t2987 * t1634 * t15234 - 0.10389515463408878255e3_f64 * t11466 * t6209 * t3006 - 0.12304822629859687989e5_f64 * t41759 * t19310 * t2988 - 0.11696447245269292414e1_f64 * t2987 * t6206 * t3006 - 0.10389515463408878255e3_f64 * t11466 * t19303 * t2988 + 0.17315859105681463759e2_f64 * t3012 * t19303 * t3006 + 0.10254018858216406658e4_f64 * t11507 * t64043 * t2988 + 0.34631718211362927518e2_f64 * t3012 * t4711 * t15234 + 0.10254018858216406658e4_f64 * t11507 * t19310 * t3006 + 0.17315859105681463759e2_f64 * t41751 * t6209 + 2.0_f64 * t64055 * t955 - t63226 - t63228 - t63579 - t63581 + 1.0_f64 * t19173 * t2963 + 0.32163958997385070134e2_f64 * t64060 * t2971 + 6.0_f64 * t2968 * t6174 * t2944 + 0.34631718211362927518e2_f64 * t11461 * t19304;
    t64068
}
