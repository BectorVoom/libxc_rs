//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1232/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1232<F: Float>(t2367: F, t6789: F, t6145: F, t20181: F, t20188: F, t20192: F, t20196: F, t20215: F, t20221: F, t20228: F, t20234: F, t20246: F, t20249: F, t20846: F, t20848: F, t20855: F, t20859: F, t20863: F, t20875: F, t20883: F, t20885: F, t20904: F, t20906: F, t2112: F, t21674: F, t21682: F, t21687: F, t21689: F, t21690: F, t21693: F, t21694: F, t21696: F, t21697: F, t21702: F, t21704: F, t21705: F, t21708: F, t21709: F, t21711: F, t21712: F, t21724: F, t21727: F, t21733: F, t21737: F, t21742: F, t21747: F, t2384: F, t335: F, t338: F, t339: F, t353: F, t376: F, t4385: F, t4402: F, t4419: F, t6128: F, t6793: F, t859: F, t892: F, t939: F) -> F {
    let t21750 = t2367 * t6789;
    let t21752 = t2367 * t6145;
    let t21756 = -F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t21674 - t335 * t338 * t892 * t6128 / F::cast_from(4.0_f64) + F::cast_from(455.0_f64) / F::cast_from(162.0_f64) * t21682 + t2384 * t4419 / F::cast_from(16.0_f64) + t335 * t338 * t339 * (-t20192 - t20246 + t21687 - t20234 - t20885 + t20904 + t20906 - t20859 + t21711 + t21712 + t20188 - t20855 + t20846 + t20848 - t20181 - t20249 - t20875 + t20196 - t20883 + t20863 + t21697 + t20228 + t20221 + t21708 + t21709 + t21689 + t21690 + t21702 + t21704 + t21705 + t21693 + t21694 + t21696 - t20215) * t376 / F::cast_from(96.0_f64) - t4385 * t21724 / F::cast_from(8.0_f64) - t21727 * t859 * t353 * t939 * t2112 / F::cast_from(8.0_f64) + F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t21733 + t4385 * t21737 / F::cast_from(16.0_f64) + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t6793 * t21742 + t6793 * t21747 / F::cast_from(4.0_f64) + F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t21750 - F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t21752 - t2384 * t4402 / F::cast_from(16.0_f64);
    t21756
}
