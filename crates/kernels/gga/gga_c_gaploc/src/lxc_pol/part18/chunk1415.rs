//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1415/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1415<F: Float>(t2487: F, t34321: F, t6711: F, t31190: F, t31213: F, t31215: F, t31217: F, t34950: F, t34953: F, t34954: F, t34957: F, t34959: F, t34962: F, t34964: F, t34967: F, t34970: F, t34973: F, t34976: F) -> F {
    let t34979 = F::cast_from(0.87421871174939309262e2_f64) * t2487 * t6711 * t34321;
    let t34980 = t34950 + t34953 - t31190 - t34954 - t31213 - t31215 + t31217 + t34957 - t34959 + t34962 - t34964 - t34967 - t34970 + t34973 - t34976 + t34979;
    t34980
}
