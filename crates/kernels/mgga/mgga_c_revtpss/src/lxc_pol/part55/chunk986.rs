//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 986/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk986<F: Float>(t2055: F, t34321: F, t7983: F, t8692: F, t2107: F, t33651: F, t2014: F, t2042: F, t8118: F, t2113: F, t7950: F, t7953: F, t1916: F, t8731: F, t1518: F, t1936: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t34323 = 2.0 * t34321 * t2055;
    let t34325 = 2.0 * t8692 * t7983;
    let t34328 = t2107 * t33651;
    let t34329 = t2014 * t34328;
    let t34346 = 3.0 * t8118 * t2042;
    let t34348 = 6.0 * t2113 * t7950;
    let t34350 = 3.0 * t2113 * t7953;
    let t34358 = 6.0 * t1916 * t8731;
    let t34359 = t1518 * t2055;
    let t34360 = t34359 * t1936;
    (t34323, t34325, t34328, t34329, t34346, t34348, t34350, t34358, t34359, t34360)
}
