//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1658/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1658<F: Float>(t43854: F, t43883: F, t43886: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t43899: F, t43902: F, t43905: F, t45232: F) -> F {
    let t45244 = t45232 - F::cast_from(0.41095999999999999998e0_f64) * t43854 + F::cast_from(0.9132444444444444444e-1_f64) * t43883 + F::cast_from(0.2283111111111111111e0_f64) * t43886 - F::cast_from(0.71030123456790123454e-1_f64) * t43888 + F::cast_from(0.45662222222222222221e-1_f64) * t43890 + F::cast_from(0.9132444444444444444e-1_f64) * t43892 - F::cast_from(0.13698666666666666667e0_f64) * t43894 - F::cast_from(0.22831111111111111111e-1_f64) * t43896 - F::cast_from(0.41095999999999999999e0_f64) * t43899 + F::cast_from(0.41096e0_f64) * t43902 + F::cast_from(0.17123333333333333333e-1_f64) * t43905;
    t45244
}
