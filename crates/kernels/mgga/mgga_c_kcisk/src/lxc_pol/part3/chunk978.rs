//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 978/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk978<F: Float>(t1413: F, t4295: F, t1489: F, t4189: F, t4223: F, t140: F, t299: F, t4291: F, t446: F, t480: F, t1460: F, t306: F, sigma0: F) -> (F, F, F, F, F) {
    let t14398 = t4295 * t1413;
    let t14399 = t14398 * sigma0;
    let t14400 = t14399 * t1489;
    let t14402 = t4223 * t4189;
    let t14405 = t140 * t299 * t4291;
    let t14409 = F::cast_from(0.11791604938271604938e-1_f64) * t140 * t446 * t480;
    let t14434 = t1460 * t306;
    (t14400, t14402, t14405, t14409, t14434)
}
