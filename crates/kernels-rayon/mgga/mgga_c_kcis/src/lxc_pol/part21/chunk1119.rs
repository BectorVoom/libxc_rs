//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1119/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1119(t26918: f64, t26920: f64, t26922: f64, t26925: f64, t26927: f64, t26931: f64, t26934: f64, t26936: f64, t26939: f64, t26942: f64, t26944: f64, t26947: f64) -> f64 {
    let t27133 = -0.9375e-1_f64 * t26918 + 0.9375e-1_f64 * t26920 + 0.91666666666666666667e0_f64 * t26922 - 0.33333333333333333334e0_f64 * t26925 - 0.21583333333333333334e0_f64 * t26927 + 0.53958333333333333334e-1_f64 * t26931 - 0.1875e0_f64 * t26934 - 0.5e0_f64 * t26936 + 0.125e0_f64 * t26939 + 0.625e-1_f64 * t26942 - 0.20234375e-1_f64 * t26944 - 0.20833333333333333333e-1_f64 * t26947;
    t27133
}
