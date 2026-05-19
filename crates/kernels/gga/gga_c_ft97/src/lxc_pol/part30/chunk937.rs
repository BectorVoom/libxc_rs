//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 937/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk937<F: Float>(t33425: F, t683: F, t173: F, t33403: F, t27616: F, t6037: F, t1614: F, t218: F, t679: F, t24286: F, t7470: F, t6815: F) -> (F, F, F, F, F, F, F) {
    let t140885 = t33425 * t683;
    let t140892 = t33403 * t173;
    let t140894 = t27616 * t140892 * t6037;
    let t140919 = t1614 * t218;
    let t140920 = t140919 * t679;
    let t140927 = t7470 * t24286;
    let t140929 = F::cast_from(0.75685073759570552987e-4_f64) * t6815 * t140927;
    (t140885, t140892, t140894, t140919, t140920, t140927, t140929)
}
