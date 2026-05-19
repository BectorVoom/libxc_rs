//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 456/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk456<F: Float>(t1142: F, t1820: F, t1205: F, t1664: F, t1214: F, t1217: F, t1671: F, t1674: F, t1677: F, t1220: F) -> (F, F, F, F) {
    let t1821 = t1142 * t1820;
    let t1823 = -t1205 - F::cast_from(0.17123333333333333333e-1_f64) * t1664;
    let t1830 = F::new(0.3529725e1) * t1671 - t1214 - F::new(0.516475e0) * t1664 + F::new(0.6311625e0) * t1674 - t1217 - F::new(0.104195e0) * t1677;
    let t1831 = t1830 * t1220;
    (t1821, t1823, t1830, t1831)
}
