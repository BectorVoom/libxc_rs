//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1122/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1122<F: Float>(t42238: F, t42242: F, t42246: F, t42258: F, t37006: F, t38149: F, t42228: F, t42234: F, t42248: F, t42250: F, t42255: F, t42260: F, t42262: F, t42264: F, t42266: F, t42268: F, t42270: F, t42272: F) -> F {
    let t44444 = F::cast_from(0.1440846329149835838e-2_f64) * t42238;
    let t44445 = F::cast_from(0.1440846329149835838e-2_f64) * t42242;
    let t44446 = F::cast_from(0.1440846329149835838e-2_f64) * t42246;
    let t44450 = F::cast_from(0.39726959900411316772e-4_f64) * t42258;
    let t44458 = F::cast_from(0.95793933614910468512e0_f64) * t37006 + F::cast_from(0.2727466165424534173e-1_f64) * t42228 - F::cast_from(0.3842256877732895568e-2_f64) * t42234 + t44444 + t44445 + t44446 + F::cast_from(0.72042316457491791901e-3_f64) * t42248 - F::cast_from(0.72042316457491791901e-3_f64) * t42250 + F::cast_from(0.85129199786595678799e-5_f64) * t42255 - t44450 - t38149 + F::cast_from(0.1702583995731913576e-4_f64) * t42260 - F::cast_from(0.5107751987195740728e-4_f64) * t42262 + F::cast_from(0.212822999466489197e-4_f64) * t42264 + F::cast_from(0.3405167991463827152e-4_f64) * t42266 - F::cast_from(0.5107751987195740728e-4_f64) * t42268 + F::cast_from(0.5107751987195740728e-4_f64) * t42270 + F::cast_from(0.2993560425465952141e-1_f64) * t42272;
    t44458
}
