//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 926/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk926<F: Float>(t23105: F, t23152: F, t23428: F, t23434: F, t4724: F, t6206: F, t981: F, t4719: F, t6227: F, t1633: F, t6189: F) -> (F, F, F, F) {
    let t23436 = t23105 + t23152 + t23428 + t23434;
    let t23446 = t4724 * t6206;
    let t23448 = F::cast_from(0.35089341735807877242e1_f64) * t981 * t23446;
    let t23450 = F::cast_from(0.51947577317044391276e2_f64) * t4719 * t6227;
    let t23451 = t6189 * t1633;
    (t23436, t23448, t23450, t23451)
}
