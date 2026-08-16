//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2957/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2957<F: Float>(t23452: F, t974: F, t981: F, t15258: F, t19467: F, t4708: F, t6226: F, t19049: F, t4734: F, t1699: F, t5023: F, t68207: F, t77657: F, t78417: F, t78422: F, t78426: F, t78428: F, t78432: F) -> (F, F, F, F, F) {
    let t78435 = F::cast_from(0.14035736694323150897e2_f64) * t981 * t23452 * t974;
    let t78438 = F::cast_from(0.51947577317044391277e2_f64) * t981 * t19467 * t15258;
    let t78441 = F::cast_from(0.10526802520742363173e2_f64) * t981 * t6226 * t4708;
    let t78443 = F::cast_from(0.51947577317044391276e2_f64) * t19049 * t4734;
    let t78444 = -F::cast_from(3.0_f64) * t1699 * t5023 * t68207 + t77657 - t78417 + t78422 - t78426 - t78428 - t78432 + t78435 - t78438 - t78441 - t78443;
    (t78435, t78438, t78441, t78443, t78444)
}
