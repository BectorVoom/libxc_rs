//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3411/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3411<F: Float>(t11509: F, t6205: F, t19247: F, t945: F, t2967: F, t6152: F, t11461: F, t11466: F, t11507: F, t15234: F, t1634: F, t19173: F, t19303: F, t19304: F, t19310: F, t2944: F, t2963: F, t2968: F, t2971: F, t2987: F, t2988: F, t3006: F, t3012: F, t41751: F, t41759: F, t4711: F, t6174: F, t6206: F, t6209: F, t63226: F, t63228: F, t63579: F, t63581: F, t955: F) -> F {
    let t64043 = t6205 * t11509;
    let t64055 = t19247 * t945;
    let t64060 = t6152 * t2967;
    let t64068 = -F::cast_from(0.23392894490538584828e1_f64) * t2987 * t1634 * t15234 - F::cast_from(0.10389515463408878255e3_f64) * t11466 * t6209 * t3006 - F::cast_from(0.12304822629859687989e5_f64) * t41759 * t19310 * t2988 - F::cast_from(0.11696447245269292414e1_f64) * t2987 * t6206 * t3006 - F::cast_from(0.10389515463408878255e3_f64) * t11466 * t19303 * t2988 + F::cast_from(0.17315859105681463759e2_f64) * t3012 * t19303 * t3006 + F::cast_from(0.10254018858216406658e4_f64) * t11507 * t64043 * t2988 + F::cast_from(0.34631718211362927518e2_f64) * t3012 * t4711 * t15234 + F::cast_from(0.10254018858216406658e4_f64) * t11507 * t19310 * t3006 + F::cast_from(0.17315859105681463759e2_f64) * t41751 * t6209 + F::new(2.0) * t64055 * t955 - t63226 - t63228 - t63579 - t63581 + F::new(1.0) * t19173 * t2963 + F::cast_from(0.32163958997385070134e2_f64) * t64060 * t2971 + F::new(6.0) * t2968 * t6174 * t2944 + F::cast_from(0.34631718211362927518e2_f64) * t11461 * t19304;
    t64068
}
