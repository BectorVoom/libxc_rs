//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1154/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1154(t10157: f64, t10160: f64, t10163: f64, t10166: f64, t10169: f64, t10176: f64, t14280: f64, t14290: f64, t14294: f64, t14297: f64, t14299: f64, t1445: f64, t4071: f64, t4078: f64, t5715: f64, t5775: f64) -> f64 {
    let t14302 = -t10157 - 0.13009920719177044025e-1_f64 * t14280 - 0.13170898365871023197e1_f64 * t4071 * t5775 + 0.13170898365871023197e1_f64 * t5715 * t4078 - 0.14634331517634470219e-1_f64 * t10160 + 0.13009920719177044025e-2_f64 * t10163 + 0.23131639038696784278e-2_f64 * t10166 + 0.9757440539382783019e-2_f64 * t10169 - 0.73171657588172351096e-2_f64 * t14290 - 0.19514881078765566038e-1_f64 * t10176 + 0.11565819519348392139e-2_f64 * t14294 + 0.65049603595885220126e-3_f64 * t14297 - 0.13170898365871023197e1_f64 * t14299 * t1445;
    t14302
}
