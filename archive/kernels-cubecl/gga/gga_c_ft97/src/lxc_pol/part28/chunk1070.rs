//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1070/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1070<F: Float>(t136226: F, t136229: F, t144892: F, t144895: F, t144899: F, t144904: F, t144908: F, t144912: F, t144917: F, t144919: F, t144923: F, t144926: F, t144930: F, t144933: F, t144935: F, t144941: F) -> F {
    let t145824 = -t144892 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t144895 - F::cast_from(2.0_f64) * t144899 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t144904 - F::cast_from(2.0_f64) * t144908 + t144912 + t144917 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t144919 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t144923 + t144926 / F::cast_from(18.0_f64) - t136226 / F::cast_from(3.0_f64) + t144930 / F::cast_from(18.0_f64) - t144933 / F::cast_from(3.0_f64) - t144935 / F::cast_from(9.0_f64) + t136229 / F::cast_from(18.0_f64) - t144941 / F::cast_from(6.0_f64);
    t145824
}
