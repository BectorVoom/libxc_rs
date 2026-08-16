//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1020/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1020<F: Float>(t8418: F, t8423: F, t8428: F, t8433: F, t8438: F, t8444: F, t8448: F, t8452: F, t8458: F, t10265: F, t10266: F, t37082: F, t37083: F, t37086: F, t7303: F, t7307: F, t7318: F, t8050: F, t8056: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42390 = F::cast_from(0.3405167991463827152e-4_f64) * t8418;
    let t42391 = F::cast_from(0.1702583995731913576e-4_f64) * t8423;
    let t42392 = F::cast_from(0.5107751987195740728e-4_f64) * t8428;
    let t42393 = F::cast_from(0.5107751987195740728e-4_f64) * t8433;
    let t42394 = F::cast_from(0.1702583995731913576e-4_f64) * t8438;
    let t42395 = F::cast_from(0.1702583995731913576e-4_f64) * t8444;
    let t42396 = F::cast_from(0.1702583995731913576e-4_f64) * t8448;
    let t42397 = F::cast_from(0.1702583995731913576e-4_f64) * t8452;
    let t42399 = F::cast_from(0.212822999466489197e-4_f64) * t8458;
    let t42403 = t42399 - t8050 + t37082 - t37083 - F::cast_from(0.12195059916630011325e-2_f64) * t7303 - F::cast_from(0.12195059916630011325e-2_f64) * t7307 + t37086 + t10265 - t10266 + t8056 - F::cast_from(0.72732431077987577948e-1_f64) * t7318;
    (t42390, t42391, t42392, t42393, t42394, t42395, t42396, t42397, t42403)
}
