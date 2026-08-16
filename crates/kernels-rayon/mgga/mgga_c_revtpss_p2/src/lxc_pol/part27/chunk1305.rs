//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1305/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1305(t1203: f64, t1214: f64, t12607: f64, t12647: f64, t1295: f64, t13107: f64, t13165: f64, t13166: f64, t2142: f64, t2148: f64, t2152: f64, t26884: f64, t26896: f64, t26927: f64, t26928: f64, t26976: f64, t26987: f64, t26991: f64, t27025: f64, t3584: f64, t3790: f64, t7602: f64, t7627: f64, t7632: f64, t7636: f64, t7637: f64, t7643: f64, t7651: f64, t7652: f64, t7666: f64, t96927: f64, t96929: f64, t96933: f64, t96938: f64, t96953: f64, t96954: f64) -> f64 {
    let t96964 = -0.4336814094102599731e0_f64 * t2148 * t13107 * t2152 - 0.65854491829355115987e0_f64 * t7632 * t13166 + 0.39512695097613069591e1_f64 * t26976 * t12647 + 0.26020884564615598386e1_f64 * t7651 * t7652 * t7627 * t3790 + 0.19756347548806534796e1_f64 * t7602 * t12607 - 0.10408353825846239354e2_f64 * t96927 * t26896 * t96929 - 0.39512695097613069591e1_f64 * t96933 * t1295 - 0.13010442282307799193e1_f64 * t26991 * t7666 - 0.19756347548806534796e1_f64 * t96938 * t1295 - 0.52041769129231196772e1_f64 * t7643 * t7652 * t26987 * t1214 + 0.26020884564615598386e1_f64 * t7643 * t7637 * t7627 * t3584 + 0.8673628188205199462e0_f64 * t7651 * t7652 * t2142 * t13165 + 0.10408353825846239354e2_f64 * t96953 * t26927 * t96954 + 0.10408353825846239354e2_f64 * t27025 * t26928 - 0.26020884564615598386e1_f64 * t7636 * t7637 * t26884 * t1203;
    t96964
}
