//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 918/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk918<F: Float>(t17231: F, t1680: F, t1806: F, t1820: F, t4887: F, t5125: F, t5126: F, t5312: F, t17205: F, t17208: F, t17211: F, t17215: F, t17219: F, t17222: F, t17225: F, t17229: F) -> (F, F, F, F, F) {
    let t17232 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t17231;
    let t17234 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t1680 * t1806;
    let t17236 = t1820 * t5125 * t4887;
    let t17237 = F::cast_from(64.0_f64) / F::cast_from(45.0_f64) * t17236;
    let t17238 = t5312 * t5126;
    let t17239 = F::cast_from(128.0_f64) / F::cast_from(45.0_f64) * t17238;
    let t17240 = t17205 + t17208 + t17211 + t17215 - t17219 + t17222 - t17225 + t17229 - t17232 + t17234 + t17237 + t17239;
    (t17232, t17234, t17237, t17239, t17240)
}
