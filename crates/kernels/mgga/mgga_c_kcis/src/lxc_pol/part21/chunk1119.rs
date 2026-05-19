//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1119/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1119<F: Float>(t26918: F, t26920: F, t26922: F, t26925: F, t26927: F, t26931: F, t26934: F, t26936: F, t26939: F, t26942: F, t26944: F, t26947: F) -> F {
    let t27133 = -F::new(0.9375e-1) * t26918 + F::new(0.9375e-1) * t26920 + F::cast_from(0.91666666666666666667e0_f64) * t26922 - F::cast_from(0.33333333333333333334e0_f64) * t26925 - F::cast_from(0.21583333333333333334e0_f64) * t26927 + F::cast_from(0.53958333333333333334e-1_f64) * t26931 - F::new(0.1875e0) * t26934 - F::new(0.5e0) * t26936 + F::new(0.125e0) * t26939 + F::new(0.625e-1) * t26942 - F::new(0.20234375e-1) * t26944 - F::cast_from(0.20833333333333333333e-1_f64) * t26947;
    t27133
}
