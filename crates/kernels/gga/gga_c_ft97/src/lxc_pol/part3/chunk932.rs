//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 932/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk932<F: Float>(t10007: F, t18412: F, t14163: F, t17785: F, t5053: F, t729: F, t773: F, t18139: F, t265: F, t1882: F, t5176: F, t13872: F, t13875: F, t13884: F, t13903: F, t13905: F, t13933: F, t18399: F, t18403: F, t18406: F, t18409: F, t1901: F, t446: F) -> F {
    let t18413 = t10007 * t18412;
    let t18416 = t14163 * t17785;
    let t18420 = t729 * t773 * t5053;
    let t18424 = t729 * t265 * t18139;
    let t18427 = t1882 * t5176;
    let t18429 = -F::new(8.0) / F::new(27.0) * t13872 + t13875 - t446 * t18399 / F::new(3.0) - t13884 - F::new(2.0) / F::new(9.0) * t1901 * t18403 - F::new(4.0) / F::new(9.0) * t1901 * t18406 + F::new(4.0) / F::new(27.0) * t1901 * t18409 - F::new(2.0) / F::new(9.0) * t1901 * t18413 - F::new(4.0) / F::new(9.0) * t1901 * t18416 - t446 * t18420 / F::new(3.0) - t446 * t18424 / F::new(3.0) + t13903 + t13905 - F::new(2.0) / F::new(27.0) * t18427 + t13933;
    t18429
}
