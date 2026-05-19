//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 380/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk380<F: Float>(t292: F, t1472: F, t4094: F, t4099: F, t4104: F, t6045: F, t6057: F, t6229: F, t6233: F, t6242: F, t6243: F, t6249: F, t6251: F, t6255: F, t6256: F) -> F {
    let t293 = F::new(0.1e-59) < t292;
    let t6260 = piecewise3::<F>(t293, F::cast_from(0.45306850413028723348e0_f64) * t4094 * t6229 - F::cast_from(0.22653425206514361674e0_f64) * t4099 * t6233 - F::cast_from(0.45306850413028723348e0_f64) * t4104 * t6229 + F::cast_from(0.22653425206514361674e0_f64) * t1472 * t6233 - F::cast_from(0.10001700163888888889e0_f64) * t6242 * t6045 * t6243 + F::cast_from(0.10001700163888888889e0_f64) * t6249 * t6251 - t6255 - F::cast_from(0.16669500273148148149e-1_f64) * t6256 * t6057, F::new(0.0));
    t6260
}
