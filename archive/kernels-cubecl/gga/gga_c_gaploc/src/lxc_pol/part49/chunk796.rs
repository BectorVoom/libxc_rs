//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 796/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk796<F: Float>(t12661: F, t13050: F, t13054: F, t13057: F, t13060: F, t13061: F, t13062: F, t13855: F, t13859: F, t13863: F, t13867: F, t13874: F, t13878: F, t13882: F, t13886: F) -> F {
    let t13887 = t13855 - t13050 - F::cast_from(0.76685851907841499354e0_f64) * t12661 + t13054 - t13057 - t13060 - F::cast_from(0.46011511144704899612e1_f64) * t13859 + F::cast_from(0.11502877786176224903e2_f64) * t13863 - F::cast_from(0.69017266717057349418e1_f64) * t13867 + t13061 - t13062 - t13874 + t13878 + t13882 - t13886;
    t13887
}
