//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1251/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1251<F: Float>(t33: F, t3881: F, t6416: F, t1113: F, t1348: F, t20256: F, t21956: F, t2255: F, t5582: F, t21955: F, t1882: F, t1892: F, t4003: F, t5658: F, t10032: F, t10035: F, t10044: F, t1399: F, t14116: F, t14120: F, t14126: F, t14131: F, t14146: F, t14149: F, t14158: F, t14161: F, t14166: F, t4118: F, t5735: F, t5745: F, t5755: F, t6844: F, t820: F, zeta_threshold: F) -> (F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t21961 = t3881 * t6416;
    let t21967 = piecewise3(t34, 0.0, 8.0 / 27.0 * t21956 * t1113 + 8.0 / 9.0 * t5582 * t2255 - 2.0 / 9.0 * t21961 * t1113 + 2.0 / 3.0 * t1348 * t20256);
    let t21969 = t21955 / 2.0 + t21967 / 2.0;
    let t21981 = t1892 * t1882;
    let t21990 = t4003 * t5658;
    let t21998 = -t14116 + 0.13009920719177044025e-2 * t14120 + t14126 + t14131 - 0.13170898365871023197e1 * t5755 * t21981 * t1399 + 0.73171657588172351096e-2 * t10032 + t10035 + 0.39029762157531132076e-1 * t14146 - 0.65049603595885220126e-3 * t10044 - 0.14634331517634470219e-1 * t14149 + t14158 + 0.23131639038696784278e-2 * t14161 + 0.26341796731742046394e1 * t5745 * t5735 * t21990 + 0.14634331517634470219e-1 * t14166 - 0.65854491829355115987e0 * t820 * t4118 * t6844;
    (t21969, t21981, t21990, t21998)
}
