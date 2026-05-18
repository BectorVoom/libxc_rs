//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 993/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk993<F: Float>(t1086: F, t7739: F, t11990: F, t2597: F, t7503: F, t11320: F, t325: F, t11938: F, t11183: F, t11186: F, t11190: F, t11193: F, t11196: F, t11200: F, t11205: F, t11212: F, t11218: F, t11220: F, t11225: F, t11229: F, t11231: F) -> (F, F, F, F, F, F, F) {
    let t11991 = t1086 * t7739;
    let t11992 = t11990 * t11991;
    let t11994 = t2597 * t7503;
    let t11995 = t11990 * t11994;
    let t11997 = t325 * t11320;
    let t11998 = t11997 * t11938;
    let t12312 = F::new(0.10862280351692200478e-4) * t11183 + F::new(0.10862280351692200478e-4) * t11186 - F::new(0.2429468532550759923e-3) * t11190 - F::new(0.2429468532550759923e-3) * t11193 - F::new(0.809822844183586641e-4) * t11196 + F::new(0.17379648562707520765e-4) * t11200 + F::new(0.50613927761474165061e-5) * t11205 - F::new(0.36207601172307334926e-6) * t11212 + F::new(0.47522476538653377091e-5) * t11218 - F::new(0.17379648562707520765e-3) * t11220 - F::new(0.17379648562707520765e-3) * t11225 + F::new(0.50613927761474165061e-5) * t11229 + F::new(0.6951859425083008306e-3) * t11231;
    (t11991, t11992, t11994, t11995, t11997, t11998, t12312)
}
