//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1102/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1102<F: Float>(t24330: F, t27546: F, t27548: F, t108472: F, t108504: F, t109017: F, t109024: F, t109030: F, t109033: F, t109038: F, t109047: F, t1096: F, t1113: F, t13520: F, t231: F, t232: F, t2409: F, t24265: F, t24276: F, t24278: F, t2428: F, t24283: F, t27704: F, t3774: F, t6023: F, t6045: F, t66105: F, t96607: F, t96692: F, t96700: F, t96703: F) -> (F,) {
    let t109055 = 0.51074886703703703704e-1 * t27546 * t24330 * t27548;
    let t109059 = -t109017 - 0.98978452595430188148e-4 * t96692 - 0.4945510644553639738e-5 * t96700 + 0.85124811172839506173e-2 * t96703 + 0.89080607335887169332e-4 * t24265 * t232 * t66105 + 0.133620911003830754e-2 * t96607 * t232 * t109024 - t109030 + 0.46509801892875584e-2 * t27704 * t24283 - 0.29693535778629056444e-3 * t24276 * t109033 * t108504 + 0.49489226297715094074e-4 * t109038 - 0.14846767889314528222e-3 * t24276 * t24278 * t1096 * t2409 + 0.51690243689028715488e-5 * t13520 * t6023 * t66105 - 0.22983699016666666667e0 * t109047 * t6045 * t231 * t1113 * t2428 + t109055 + 0.62028292426834458586e-5 * t3774 * t6023 * t108472;
    (t109059,)
}
