//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1699/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1699<F: Float>(t1882: F, t1892: F, t4003: F, t5658: F, t10032: F, t10035: F, t10044: F, t1399: F, t14116: F, t14120: F, t14126: F, t14131: F, t14146: F, t14149: F, t14158: F, t14161: F, t14166: F, t4118: F, t5735: F, t5745: F, t5755: F, t6844: F, t820: F) -> (F, F, F) {
    let t21981 = t1892 * t1882;
    let t21990 = t4003 * t5658;
    let t21998 = -t14116 + F::cast_from(0.13009920719177044025e-2_f64) * t14120 + t14126 + t14131 - F::cast_from(0.13170898365871023197e1_f64) * t5755 * t21981 * t1399 + F::cast_from(0.73171657588172351096e-2_f64) * t10032 + t10035 + F::cast_from(0.39029762157531132076e-1_f64) * t14146 - F::cast_from(0.65049603595885220126e-3_f64) * t10044 - F::cast_from(0.14634331517634470219e-1_f64) * t14149 + t14158 + F::cast_from(0.23131639038696784278e-2_f64) * t14161 + F::cast_from(0.26341796731742046394e1_f64) * t5745 * t5735 * t21990 + F::cast_from(0.14634331517634470219e-1_f64) * t14166 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t4118 * t6844;
    (t21981, t21990, t21998)
}
