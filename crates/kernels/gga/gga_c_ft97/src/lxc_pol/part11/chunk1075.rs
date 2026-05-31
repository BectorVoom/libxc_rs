//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1075/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1075<F: Float>(t342: F, t657: F, t8639: F, t2252: F, t2326: F, t240: F, t9570: F, t630: F, t9507: F, t13605: F, t1526: F, t231: F, t2320: F, t2321: F, t343: F, t3806: F, t8608: F, t9512: F, t9571: F, t9692: F, t9745: F, t9757: F, t9761: F, t9781: F) -> F {
    let t42293 = F::cast_from(5.0_f64) / F::cast_from(54.0_f64) * t342 * t8639 * t657;
    let t42295 = t342 * t2252 * t2326;
    let t42307 = t240 * t9570;
    let t42320 = t342 * t630 * t9507;
    let t42322 = -t42293 + t9512 + t42295 / F::cast_from(6.0_f64) - t1526 * t2320 * t9757 / F::cast_from(4.0_f64) - t1526 * t2320 * t2321 * t8608 / F::cast_from(12.0_f64) - t1526 * t3806 * t9745 / F::cast_from(3.0_f64) - F::cast_from(7.0_f64) / F::cast_from(27.0_f64) * t1526 * t13605 * t42307 * t9571 - t1526 * t2320 * t9761 / F::cast_from(4.0_f64) - t342 * t343 * t231 * t9692 / F::cast_from(4.0_f64) + t9781 - t42320 / F::cast_from(4.0_f64);
    t42322
}
