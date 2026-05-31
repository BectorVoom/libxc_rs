//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1081/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1081<F: Float>(t1192: F, t2352: F, t2409: F, t3067: F, t4007: F, t6781: F, t2417: F, t9296: F, t13772: F, t13778: F, t13785: F, t13789: F, t13794: F, t13801: F, t13804: F, t13810: F, t13813: F, t13818: F, t13822: F, t13826: F, t13833: F, t13837: F, t2408: F, t3066: F, t3207: F, t335: F, t827: F) -> (F, F, F, F, F, F) {
    let t13840 = t1192 * t2352;
    let t13842 = t2409 * t3067 * t13840;
    let t13846 = t2409 * t6781 * t4007;
    let t13849 = t1192 * t2417;
    let t13851 = t2409 * t9296 * t13849;
    let t13854 = -t827 * t13772 / F::cast_from(48.0_f64) - t13778 / F::cast_from(192.0_f64) - t13785 / F::cast_from(768.0_f64) - t13789 / F::cast_from(3072.0_f64) - t13794 / F::cast_from(24.0_f64) + t13801 / F::cast_from(1536.0_f64) + t13804 / F::cast_from(1536.0_f64) - t13810 + t13813 / F::cast_from(96.0_f64) + t13818 / F::cast_from(96.0_f64) - t3207 * t13822 / F::cast_from(16.0_f64) - t335 * t13826 / F::cast_from(48.0_f64) + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t13833 + t2408 * t13837 / F::cast_from(24.0_f64) + t3066 * t13842 / F::cast_from(48.0_f64) + t2408 * t13846 / F::cast_from(24.0_f64) - t3066 * t13851 / F::cast_from(16.0_f64);
    (t13840, t13842, t13846, t13849, t13851, t13854)
}
