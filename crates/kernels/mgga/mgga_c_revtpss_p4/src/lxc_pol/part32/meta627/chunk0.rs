//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2002/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2002<F: Float>(t108978: F, t2047: F, t108986: F, t101230: F, t101785: F, t101955: F, t101969: F, t101972: F, t10309: F, t108966: F, t108975: F, t108983: F, t108990: F, t25162: F, t26175: F, t26182: F, t28147: F, t28628: F, t34764: F) -> F {
    let t110039 = t2047 * t108978;
    let t110044 = t2047 * t108986;
    let t110049 = -F::cast_from(40.0_f64) * t10309 * t34764 * t28147 - t101955 - t101969 - t101972 + F::cast_from(20.0_f64) * t101785 * t28147 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t101230 * t28628 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t108966 * t26182 + F::cast_from(20.0_f64) * t26175 * t108975 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t25162 * t110039 + F::cast_from(10.0_f64) * t26175 * t108983 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t25162 * t110044 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t108990 * t26182;
    t110049
}
