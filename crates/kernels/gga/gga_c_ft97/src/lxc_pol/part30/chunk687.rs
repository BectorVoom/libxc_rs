//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 687/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk687<F: Float>(t2360: F, t317: F, t3886: F, t28938: F, t1477: F, t2404: F, t2347: F, t684: F, t6970: F, t25412: F, t4309: F, t193: F) -> (F, F, F, F, F, F, F, F) {
    let t28939 = t317 * t2360;
    let t28940 = t28939 * t3886;
    let t28941 = t28938 * t28940;
    let t28944 = t2404 * t1477;
    let t28945 = t317 * t2347;
    let t28946 = t28945 * t3886;
    let t28947 = t28944 * t28946;
    let t28950 = t6970 * t684;
    let t28951 = t25412 * t28950;
    let t28954 = t1477 * t4309;
    let t28955 = t193 * t28954;
    (t28940, t28941, t28944, t28946, t28947, t28950, t28951, t28955)
}
