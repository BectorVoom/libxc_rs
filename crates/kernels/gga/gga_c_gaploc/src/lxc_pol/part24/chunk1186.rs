//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1186/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1186<F: Float>(t23741: F, t3347: F, t10215: F, t599: F, t475: F, t2268: F, t26938: F, t6767: F, t21389: F, t7937: F, t10178: F, t6305: F) -> (F, F, F, F, F, F) {
    let t31825 = F::new(0.85365019907028448797e-1) * t23741 * t3347;
    let t31828 = t599 * t10215;
    let t31829 = t31828 * t475;
    let t31835 = F::new(0.68292015925622759036e0) * t2268 * t26938 * t6767;
    let t31838 = F::new(0.68292015925622759036e0) * t2268 * t7937 * t21389;
    let t31840 = F::new(0.34146007962811379518e0) * t6305 * t10178;
    (t31825, t31828, t31829, t31835, t31838, t31840)
}
