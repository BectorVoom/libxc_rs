//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 442/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk442<F: Float>(t1307: F, t210: F, t214: F, t535: F, t792: F, t795: F, t1313: F, t1315: F) -> (F, F, F) {
    let t1317 = t210 * t214 * t1307;
    let t1322 = F::cast_from(0.41666666666666666666e-3_f64) * t792 * t535 * t795;
    let t1323 = -t1313 - F::cast_from(0.16666666666666666666e-2_f64) * t1315 * t1317 - t1322;
    (t1317, t1322, t1323)
}
