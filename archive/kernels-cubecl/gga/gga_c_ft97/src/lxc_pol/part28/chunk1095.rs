//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1095/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1095<F: Float>(t1337: F, t358: F, t1286: F, t34584: F, t376: F, t137488: F, t137547: F, t146093: F, t146376: F, t146561: F, t1557: F, t1570: F, t22907: F, t25601: F, t25609: F, t25615: F, t25622: F, t26119: F, t28: F, t3188: F, t32016: F, t3204: F, t32641: F, t432: F, t5501: F, t5507: F, t6562: F, t7162: F, t948: F) -> F {
    let t147008 = t1337 * t358;
    let t147024 = t1286 * t376 * t34584;
    let t147040 = F::cast_from(8.0_f64) * t146376 + F::cast_from(8.0_f64) * t146561 + F::cast_from(8.0_f64) * t146093 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5501 * t22907 * t147008 * t3204 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5501 * t25609 * t1337 * t1570 * t3188 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t5501 * t25615 * t1337 * t1557 * t3188 - t147024 / F::cast_from(9.0_f64) - t32016 * t26119 / F::cast_from(18.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1286 * t28 * t5507 * t6562 * t432 - t948 * t32641 - t137547 / F::cast_from(18.0_f64) - t5501 * t137488 * t25601 / F::cast_from(3.0_f64) + t7162 * t25622 / F::cast_from(6.0_f64);
    t147040
}
