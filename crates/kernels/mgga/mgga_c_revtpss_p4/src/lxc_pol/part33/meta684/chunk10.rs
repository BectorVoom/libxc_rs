//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2261/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2261<F: Float>(t2142: F, t6564: F, t30840: F, t460: F, t1769: F, t1828: F, t104510: F, t105134: F, t105404: F, t105576: F, t1214: F, t1294: F, t1295: F, t1774: F, t1775: F, t20744: F, t21382: F, t21408: F, t2151: F, t26937: F, t26994: F, t26999: F, t29174: F, t29187: F, t29227: F, t29275: F, t30747: F, t30853: F, t30887: F, t5498: F, t6588: F, t7602: F, t7632: F, t7636: F, t7637: F, t7643: F, t7652: F, t97066: F, t97304: F) -> F {
    let t112706 = t6564 * t2142;
    let t112714 = t460 * t30840;
    let t112721 = t1769 * t1828;
    let t112744 = -F::cast_from(0.65854491829355115987e0_f64) * t26999 * t6588 - F::cast_from(0.13170898365871023197e1_f64) * t29227 * t5498 + F::cast_from(0.13170898365871023197e1_f64) * t7602 * t21382 + F::cast_from(0.17347256376410398924e1_f64) * t26937 * t30887 - F::cast_from(0.65854491829355115987e0_f64) * t112706 * t1295 + F::cast_from(0.26341796731742046394e1_f64) * t7632 * t21408 + F::cast_from(0.34694512752820797848e1_f64) * t97304 * t30853 * t104510 - F::cast_from(0.65854491829355115987e0_f64) * t112714 * t1295 - F::cast_from(0.26341796731742046394e1_f64) * t105576 * t20744 - F::cast_from(0.17347256376410398924e1_f64) * t29275 * t29187 - F::cast_from(0.69389025505641595696e1_f64) * t97066 * t2151 * t112721 * t1214 - F::cast_from(0.10408353825846239354e2_f64) * t105404 * t2151 * t112721 * t1294 + F::cast_from(0.34694512752820797848e1_f64) * t26994 * t7637 * t29174 * t1774 + F::cast_from(0.34694512752820797848e1_f64) * t7636 * t7652 * t29174 * t1828 - F::cast_from(0.34694512752820797848e1_f64) * t7643 * t7652 * t30747 * t1294 - F::cast_from(0.13170898365871023197e1_f64) * t105134 * t1775;
    t112744
}
