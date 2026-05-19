//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1259/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1259<F: Float>(t31682: F, t31684: F, t35949: F, t35955: F, t35959: F, t37801: F, t37807: F, t37808: F, t37810: F, t37811: F, t37813: F, t37814: F, t37815: F, t37816: F, t37817: F, t40385: F, t40387: F, t40390: F) -> F {
    let t42075 = F::cast_from(0.5590771962596293701e-2_f64) * t31682 + F::cast_from(0.16006300097412701803e-1_f64) * t40385 - F::cast_from(0.17149607247227894789e-2_f64) * t40387 - F::cast_from(0.62896184579208304138e-3_f64) * t31684 - F::cast_from(0.34299214494455789578e-2_f64) * t35949 - t37801 + F::cast_from(0.42874018118069736972e-3_f64) * t35955 + F::cast_from(0.34299214494455789578e-2_f64) * t35959 + t37807 + t37808 - t37810 - F::cast_from(0.13719685797782315831e-1_f64) * t40390 + t37811 - t37813 - t37814 + t37815 - t37816 + t37817;
    t42075
}
