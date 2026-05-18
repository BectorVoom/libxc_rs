//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 930/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk930<F: Float>(t385: F, t13593: F, t13759: F, t67: F, t1389: F, t381: F, t916: F, t1286: F, t3777: F, t1284: F, t13423: F, t1280: F, t1287: F, t13394: F, t340: F, t379: F, t382: F, t4134: F, t4144: F, t4148: F, t6141: F, t6142: F) -> (F, F, F, F) {
    let t386 = t385 < -F::new(0.66725e-1);
    let t13761 = t67 * (t13593 + t13759);
    let t13776 = F::new(1.0) / t381 / t916 / t1389;
    let t13777 = t3777 * t1286;
    let t13778 = t13776 * t13777;
    let t13785 = t1284 * t13423;
    let t13790 = piecewise3::<f64>(t386, F::new(0.0), F::new(10.0) / F::new(9.0) * t340 * t13761 * t382 - F::new(10.0) / F::new(9.0) * t340 * t4134 * t1287 + F::new(40.0) / F::new(27.0) * t340 * t1280 * t4144 - F::new(10.0) / F::new(9.0) * t340 * t1280 * t4148 - F::new(280.0) / F::new(243.0) * t340 * t379 * t13778 + F::new(40.0) / F::new(27.0) * t6141 * t6142 * t13394 - F::new(10.0) / F::new(27.0) * t340 * t379 * t13785);
    (t13777, t13778, t13785, t13790)
}
