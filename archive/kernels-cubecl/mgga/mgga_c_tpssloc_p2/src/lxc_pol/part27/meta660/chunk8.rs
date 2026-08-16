//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2313/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2313<F: Float>(t1824: F, t6955: F, t2006: F, t5286: F, t1338: F, t26328: F, t26462: F, t6914: F, t22705: F, t26414: F, t81228: F, t26415: F, t81159: F) -> (F, F, F, F, F, F) {
    let t90942 = t6955 * t1824;
    let t90946 = t2006 * t5286;
    let t90952 = t1338 * t26328;
    let t90956 = t6914 * t26462;
    let t90957 = F::cast_from(0.38381794893125283518e-1_f64) * t90956;
    let t90961 = t81228 * t22705 * t26414;
    let t90962 = F::cast_from(0.16449340668482264365e-1_f64) * t90961;
    let t90963 = t81159 * t26415;
    (t90942, t90946, t90952, t90957, t90962, t90963)
}
