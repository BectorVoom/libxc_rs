//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1103/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1103<F: Float>(t2107: F, t33651: F, t2014: F, t2042: F, t8118: F, t2113: F, t7950: F, t7953: F, t1916: F, t8731: F, t1518: F, t2055: F) -> (F, F, F, F, F, F, F) {
    let t34328 = t2107 * t33651;
    let t34329 = t2014 * t34328;
    let t34346 = F::cast_from(3.0_f64) * t8118 * t2042;
    let t34348 = F::cast_from(6.0_f64) * t2113 * t7950;
    let t34350 = F::cast_from(3.0_f64) * t2113 * t7953;
    let t34358 = F::cast_from(6.0_f64) * t1916 * t8731;
    let t34359 = t1518 * t2055;
    (t34328, t34329, t34346, t34348, t34350, t34358, t34359)
}
