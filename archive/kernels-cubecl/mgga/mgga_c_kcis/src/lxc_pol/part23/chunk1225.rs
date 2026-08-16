//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1225/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1225<F: Float>(t1506: F, t97900: F, t97919: F, t97939: F, t97958: F, t28644: F, t4184: F, t17708: F, t7940: F, t17308: F, t7962: F, t12335: F, t8207: F) -> (F, F, F, F, F) {
    let t97961 = t1506 * (t97900 + t97919 + t97939 + t97958);
    let t97976 = F::cast_from(2.0_f64) * t4184 * t28644;
    let t97977 = t7940 * t17708;
    let t97979 = F::cast_from(2.0_f64) * t17308 * t7962;
    let t97984 = t12335 * t8207;
    (t97961, t97976, t97977, t97979, t97984)
}
