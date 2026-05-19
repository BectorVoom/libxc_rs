//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 357/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk357<F: Float>(t1506: F, t2069: F, t1557: F, t1891: F, t1566: F, t1569: F, t1898: F, t1901: F, t1904: F, t1572: F) -> (F, F, F, F) {
    let t2070 = t1506 * t2069;
    let t2072 = -t1557 - F::cast_from(0.17123333333333333333e-1_f64) * t1891;
    let t2079 = F::new(0.3529725e1) * t1898 - t1566 - F::new(0.516475e0) * t1891 + F::new(0.6311625e0) * t1901 - t1569 - F::new(0.104195e0) * t1904;
    let t2080 = t2079 * t1572;
    (t2070, t2072, t2079, t2080)
}
