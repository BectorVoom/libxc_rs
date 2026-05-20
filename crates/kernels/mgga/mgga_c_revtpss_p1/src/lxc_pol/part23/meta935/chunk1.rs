//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3075/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3075<F: Float>(t1196: F, t5184: F, t68680: F, t1187: F, t6534: F, t1757: F, t58708: F, t20400: F, t5198: F, t20887: F, t5192: F, t58665: F) -> (F, F, F, F, F) {
    let t81322 = F::cast_from(0.51947577317044391277e2_f64) * t1196 * t68680 * t5184;
    let t81323 = t6534 * t1187;
    let t81326 = F::cast_from(0.10526802520742363173e2_f64) * t58708 * t1757 * t81323;
    let t81328 = F::cast_from(0.35089341735807877242e1_f64) * t20400 * t5198;
    let t81330 = F::cast_from(0.35089341735807877242e1_f64) * t5192 * t20887;
    let t81333 = F::cast_from(0.31168546390226634766e3_f64) * t58665 * t5184 * t81323;
    (t81322, t81326, t81328, t81330, t81333)
}
