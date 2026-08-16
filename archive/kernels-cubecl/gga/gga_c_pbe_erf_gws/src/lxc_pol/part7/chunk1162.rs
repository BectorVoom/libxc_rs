//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1162/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1162<F: Float>(t20748: F, t6672: F, t6335: F, t6342: F, t6800: F, t6605: F, t6702: F, t6183: F, t6706: F, t2120: F, t20305: F, t20720: F, t20725: F, t20731: F, t20733: F, t20734: F, t20739: F, t20746: F, t2258: F, t2345: F, t3247: F, t6275: F, t6276: F, t6287: F, t904: F) -> (F, F, F, F, F) {
    let t20750 = t6672 * t20748 / F::cast_from(4.0_f64);
    let t20753 = t6800 * t6335 * t6342 / F::cast_from(16.0_f64);
    let t20754 = t6702 * t6605;
    let t20755 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t20754;
    let t20756 = t6183 * t6706;
    let t20757 = t2120 * t20756;
    let t20758 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t20757;
    let t20759 = -F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t3247 * t2345 * t20305 * t6287 + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t20720 - t20725 + t20731 - F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t20733 * t904 * t20734 * t2258 + t6275 * t6276 * t20739 / F::cast_from(16.0_f64) + t20746 + t20750 + t20753 + t20755 + t20758;
    (t20750, t20753, t20755, t20758, t20759)
}
