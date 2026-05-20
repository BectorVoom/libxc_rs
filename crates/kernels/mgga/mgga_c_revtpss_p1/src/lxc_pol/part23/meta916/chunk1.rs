//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2956/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2956<F: Float>(t19082: F, t4719: F, t15547: F, t6219: F, t6205: F, t972: F, t1634: F, t52877: F, t6227: F, t23694: F, t3011: F, t4733: F, t981: F) -> (F, F, F, F, F, F) {
    let t78417 = F::cast_from(0.10526802520742363173e2_f64) * t4719 * t19082;
    let t78422 = F::cast_from(0.35089341735807877242e1_f64) * t15547 * t6219;
    let t78423 = t6205 * t972;
    let t78426 = F::cast_from(0.10526802520742363173e2_f64) * t52877 * t1634 * t78423;
    let t78428 = F::cast_from(0.51947577317044391276e2_f64) * t15547 * t6227;
    let t78429 = t3011 * t23694;
    let t78432 = F::cast_from(0.17315859105681463759e2_f64) * t981 * t78429 * t4733;
    (t78417, t78422, t78423, t78426, t78428, t78432)
}
