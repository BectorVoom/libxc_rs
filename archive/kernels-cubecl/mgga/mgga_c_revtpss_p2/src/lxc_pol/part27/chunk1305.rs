//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1305/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1305<F: Float>(t1203: F, t1214: F, t12607: F, t12647: F, t1295: F, t13107: F, t13165: F, t13166: F, t2142: F, t2148: F, t2152: F, t26884: F, t26896: F, t26927: F, t26928: F, t26976: F, t26987: F, t26991: F, t27025: F, t3584: F, t3790: F, t7602: F, t7627: F, t7632: F, t7636: F, t7637: F, t7643: F, t7651: F, t7652: F, t7666: F, t96927: F, t96929: F, t96933: F, t96938: F, t96953: F, t96954: F) -> F {
    let t96964 = -F::cast_from(0.4336814094102599731e0_f64) * t2148 * t13107 * t2152 - F::cast_from(0.65854491829355115987e0_f64) * t7632 * t13166 + F::cast_from(0.39512695097613069591e1_f64) * t26976 * t12647 + F::cast_from(0.26020884564615598386e1_f64) * t7651 * t7652 * t7627 * t3790 + F::cast_from(0.19756347548806534796e1_f64) * t7602 * t12607 - F::cast_from(0.10408353825846239354e2_f64) * t96927 * t26896 * t96929 - F::cast_from(0.39512695097613069591e1_f64) * t96933 * t1295 - F::cast_from(0.13010442282307799193e1_f64) * t26991 * t7666 - F::cast_from(0.19756347548806534796e1_f64) * t96938 * t1295 - F::cast_from(0.52041769129231196772e1_f64) * t7643 * t7652 * t26987 * t1214 + F::cast_from(0.26020884564615598386e1_f64) * t7643 * t7637 * t7627 * t3584 + F::cast_from(0.8673628188205199462e0_f64) * t7651 * t7652 * t2142 * t13165 + F::cast_from(0.10408353825846239354e2_f64) * t96953 * t26927 * t96954 + F::cast_from(0.10408353825846239354e2_f64) * t27025 * t26928 - F::cast_from(0.26020884564615598386e1_f64) * t7636 * t7637 * t26884 * t1203;
    t96964
}
