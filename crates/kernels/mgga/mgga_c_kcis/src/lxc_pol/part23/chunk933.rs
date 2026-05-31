//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 933/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk933<F: Float>(t2016: F, t4188: F, t4190: F, t4310: F, t5897: F, t12335: F, t2069: F, t12338: F, t5900: F, t4184: F, t6048: F, t12345: F) -> (F, F, F, F, F, F, F, F) {
    let t17311 = t2016 * t4188;
    let t17313 = F::cast_from(2.0_f64) * t17311 * t4190;
    let t17314 = t5897 * t4310;
    let t17315 = t12335 * t2069;
    let t17317 = F::cast_from(4.0_f64) * t12338 * t5900;
    let t17319 = F::cast_from(2.0_f64) * t4184 * t6048;
    let t17320 = t2069 * t4190;
    let t17322 = F::cast_from(6.0_f64) * t12345 * t17320;
    (t17311, t17313, t17314, t17315, t17317, t17319, t17320, t17322)
}
