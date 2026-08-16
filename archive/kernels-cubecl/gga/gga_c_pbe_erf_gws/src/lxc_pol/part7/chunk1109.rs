//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1109/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1109<F: Float>(t2074: F, t2182: F, t19819: F, t19821: F, t19824: F, t19829: F, t19836: F, t19841: F, t19843: F, t19845: F, t19857: F, t2376: F, t2395: F, t2408: F, t2409: F, t2417: F, t3067: F, t3207: F, t4385: F, t6127: F, t6449: F, t6723: F, t810: F, t831: F, t9241: F, t9296: F) -> (F, F) {
    let t19859 = t2074 * t2182;
    let t19869 = -F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t19819 - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t19821 + t4385 * t19824 / F::cast_from(4.0_f64) - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t4385 * t19829 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t3207 * t2409 * t2395 * t6449 - F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t19836 - F::cast_from(455.0_f64) / F::cast_from(324.0_f64) * t19841 - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t19843 - F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t19845 + t2408 * t2409 * t9296 * t6127 * t810 / F::cast_from(2.0_f64) + t2408 * t2409 * t2376 * t6723 * t810 / F::cast_from(12.0_f64) - F::cast_from(7.0_f64) / F::cast_from(4.0_f64) * t19857 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t9241 * t2409 * t831 * t19859 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t3207 * t2409 * t3067 * t2182 * t2417;
    (t19859, t19869)
}
