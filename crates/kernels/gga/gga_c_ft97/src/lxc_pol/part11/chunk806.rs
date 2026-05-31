//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 806/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk806<F: Float>(t2440: F, t70: F, t327: F, t9570: F, t9571: F, t8640: F, t895: F, t2253: F, t2934: F, t2920: F, t2941: F, t10871: F, t10875: F, t10877: F, t10896: F, t10900: F, t10907: F, t10912: F, t2265: F, t631: F) -> (F, F, F, F) {
    let t10915 = t70 * t2440;
    let t10916 = t327 * t9570;
    let t10918 = t10915 * t10916 * t9571;
    let t10921 = t8640 * t895;
    let t10923 = t2253 * t2934;
    let t10925 = t2253 * t2920;
    let t10927 = t2253 * t2941;
    let t10929 = F::cast_from(2.0_f64) * t2265 * t10871 + t631 * t10875 - t2265 * t10877 + t631 * t10896 / F::cast_from(2.0_f64) + t631 * t10900 / F::cast_from(6.0_f64) + F::cast_from(6.0_f64) * t631 * t10907 - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t631 * t10912 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t631 * t10918 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t10921 - t10923 / F::cast_from(3.0_f64) - t10925 / F::cast_from(9.0_f64) + F::cast_from(3.0_f64) * t10927;
    (t10915, t10916, t10918, t10929)
}
