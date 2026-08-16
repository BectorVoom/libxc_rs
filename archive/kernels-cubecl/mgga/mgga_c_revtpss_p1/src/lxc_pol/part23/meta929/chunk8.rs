//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3041/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3041<F: Float>(t19137: F, t19153: F, t27717: F, t5019: F, t5023: F, t63907: F, t78339: F, t78342: F, t78703: F, t78706: F, t78709: F, t78712: F, t78715: F, t78717: F) -> F {
    let t81088 = F::cast_from(6.0_f64) * t19137 * t5019 * t5023 - F::cast_from(3.0_f64) * t19153 * t5019 * t5023 + F::cast_from(6.0_f64) * t27717 * t5023 * t63907 + t78339 + t78342 - t78703 - t78706 + t78709 - t78712 + t78715 - t78717;
    t81088
}
