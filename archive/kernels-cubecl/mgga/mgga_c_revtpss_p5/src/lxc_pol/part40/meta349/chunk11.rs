//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1200/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1200<F: Float>(t10157: F, t10160: F, t10163: F, t10166: F, t10169: F, t10176: F, t14280: F, t14290: F, t14294: F, t14297: F, t14299: F, t1445: F, t4071: F, t4078: F, t5715: F, t5775: F) -> F {
    let t14302 = -t10157 - F::cast_from(0.13009920719177044025e-1_f64) * t14280 - F::cast_from(0.13170898365871023197e1_f64) * t4071 * t5775 + F::cast_from(0.13170898365871023197e1_f64) * t5715 * t4078 - F::cast_from(0.14634331517634470219e-1_f64) * t10160 + F::cast_from(0.13009920719177044025e-2_f64) * t10163 + F::cast_from(0.23131639038696784278e-2_f64) * t10166 + F::cast_from(0.9757440539382783019e-2_f64) * t10169 - F::cast_from(0.73171657588172351096e-2_f64) * t14290 - F::cast_from(0.19514881078765566038e-1_f64) * t10176 + F::cast_from(0.11565819519348392139e-2_f64) * t14294 + F::cast_from(0.65049603595885220126e-3_f64) * t14297 - F::cast_from(0.13170898365871023197e1_f64) * t14299 * t1445;
    t14302
}
