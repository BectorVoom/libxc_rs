//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2216/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2216<F: Float>(t4719: F, t6219: F, t15101: F, t6110: F, t23466: F, t935: F, t2924: F, t19467: F, t4711: F, t981: F, t1699: F, t6400: F) -> (F, F, F, F, F, F, F) {
    let t23562 = F::cast_from(0.35089341735807877242e1_f64) * t4719 * t6219;
    let t23564 = F::cast_from(6.0_f64) * t15101 * t6110;
    let t23565 = t23466 * t935;
    let t23567 = F::cast_from(6.0_f64) * t2924 * t23565;
    let t23568 = t19467 * t4711;
    let t23570 = F::cast_from(0.51947577317044391277e2_f64) * t981 * t23568;
    let t23571 = t6400 * t1699;
    (t23562, t23564, t23565, t23567, t23568, t23570, t23571)
}
