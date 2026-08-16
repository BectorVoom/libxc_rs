//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1225/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1225(t1506: f64, t97900: f64, t97919: f64, t97939: f64, t97958: f64, t28644: f64, t4184: f64, t17708: f64, t7940: f64, t17308: f64, t7962: f64, t12335: f64, t8207: f64) -> (f64, f64, f64, f64, f64) {
    let t97961 = t1506 * (t97900 + t97919 + t97939 + t97958);
    let t97976 = 2.0_f64 * t4184 * t28644;
    let t97977 = t7940 * t17708;
    let t97979 = 2.0_f64 * t17308 * t7962;
    let t97984 = t12335 * t8207;
    (t97961, t97976, t97977, t97979, t97984)
}
