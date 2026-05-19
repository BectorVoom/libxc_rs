//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 937/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk937<F: Float>(t10310: F, t2365: F, t1429: F, t10241: F, t447: F) -> (F, F, F) {
    let t10311 = t2365 * t10310;
    let t10312 = t1429 * t10311;
    let t10313 = F::cast_from(0.14896037479937677779e-1_f64) * t10312;
    let t10314 = t10241 * t447;
    (t10311, t10313, t10314)
}
