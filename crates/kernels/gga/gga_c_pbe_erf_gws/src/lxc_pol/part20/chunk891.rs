//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 891/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk891<F: Float>(t829: F, t830: F, t9883: F, t3717: F, t831: F, t2370: F, t1115: F, t2397: F, t2408: F, t3207: F, t335: F, t3917: F, t4425: F, t4430: F, t4443: F, t827: F, t8622: F, t8641: F, t8643: F, t8646: F, t8664: F, t8666: F, t8710: F, t9865: F, t9869: F, t9873: F, t9879: F) -> (F, F) {
    let t9885 = t829 * t830 * t9883;
    let t9888 = t831 * t3717;
    let t9890 = t2370 * t830 * t9888;
    let t9893 = t3917 * t2397 / F::cast_from(96.0_f64) + t335 * t9865 / F::cast_from(48.0_f64) + t8622 + t2408 * t9869 / F::cast_from(24.0_f64) - t3207 * t9873 / F::cast_from(8.0_f64) + F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t4425 - F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t4430 - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t4443 - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t9879 + t8641 + t8643 + t8646 + t8664 - t1115 * t8710 / F::cast_from(24.0_f64) - t827 * t9885 / F::cast_from(48.0_f64) - t827 * t9890 / F::cast_from(48.0_f64) - t8666;
    (t9888, t9893)
}
