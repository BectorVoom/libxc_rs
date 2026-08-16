//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2086/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2086<F: Float>(t26462: F, t6914: F, t22705: F, t26414: F, t81228: F, t26415: F, t81159: F, t26418: F, t7736: F, t80854: F, t81064: F, t22704: F, t26410: F) -> (F, F, F, F, F, F) {
    let t90956 = t6914 * t26462;
    let t90957 = F::cast_from(0.38381794893125283518e-1_f64) * t90956;
    let t90961 = t81228 * t22705 * t26414;
    let t90962 = F::cast_from(0.16449340668482264365e-1_f64) * t90961;
    let t90963 = t81159 * t26415;
    let t90964 = F::cast_from(0.76763589786250567036e-1_f64) * t90963;
    let t90970 = t6914 * t26418;
    let t90971 = F::cast_from(0.38381794893125283518e-1_f64) * t90970;
    let t90980 = t81064 * t80854 * t7736;
    let t90983 = t22704 * t22705 * t26410;
    (t90957, t90962, t90964, t90971, t90980, t90983)
}
