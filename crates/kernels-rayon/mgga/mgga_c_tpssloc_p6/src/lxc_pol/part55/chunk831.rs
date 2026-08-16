//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 831/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk831(t483: f64, t493: f64, t470: f64, t2134: f64, t488: f64, t8875: f64) -> (f64, f64, f64) {
    let t8878 = t493 * t483;
    let t8879 = t470 * t8878;
    let t8882 = 0.40372756094140390856e-3_f64 * t2134 * t8875 + t8879 * t488 / 1536.0_f64;
    (t8878, t8879, t8882)
}
