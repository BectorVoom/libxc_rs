//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1232/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1232(t2367: f64, t6789: f64, t6145: f64, t20181: f64, t20188: f64, t20192: f64, t20196: f64, t20215: f64, t20221: f64, t20228: f64, t20234: f64, t20246: f64, t20249: f64, t20846: f64, t20848: f64, t20855: f64, t20859: f64, t20863: f64, t20875: f64, t20883: f64, t20885: f64, t20904: f64, t20906: f64, t2112: f64, t21674: f64, t21682: f64, t21687: f64, t21689: f64, t21690: f64, t21693: f64, t21694: f64, t21696: f64, t21697: f64, t21702: f64, t21704: f64, t21705: f64, t21708: f64, t21709: f64, t21711: f64, t21712: f64, t21724: f64, t21727: f64, t21733: f64, t21737: f64, t21742: f64, t21747: f64, t2384: f64, t335: f64, t338: f64, t339: f64, t353: f64, t376: f64, t4385: f64, t4402: f64, t4419: f64, t6128: f64, t6793: f64, t859: f64, t892: f64, t939: f64) -> f64 {
    let t21750 = t2367 * t6789;
    let t21752 = t2367 * t6145;
    let t21756 = -7.0_f64 / 12.0_f64 * t21674 - t335 * t338 * t892 * t6128 / 4.0_f64 + 455.0_f64 / 162.0_f64 * t21682 + t2384 * t4419 / 16.0_f64 + t335 * t338 * t339 * (-t20192 - t20246 + t21687 - t20234 - t20885 + t20904 + t20906 - t20859 + t21711 + t21712 + t20188 - t20855 + t20846 + t20848 - t20181 - t20249 - t20875 + t20196 - t20883 + t20863 + t21697 + t20228 + t20221 + t21708 + t21709 + t21689 + t21690 + t21702 + t21704 + t21705 + t21693 + t21694 + t21696 - t20215) * t376 / 96.0_f64 - t4385 * t21724 / 8.0_f64 - t21727 * t859 * t353 * t939 * t2112 / 8.0_f64 + 7.0_f64 / 24.0_f64 * t21733 + t4385 * t21737 / 16.0_f64 + 3.0_f64 / 4.0_f64 * t6793 * t21742 + t6793 * t21747 / 4.0_f64 + 7.0_f64 / 12.0_f64 * t21750 - 7.0_f64 / 12.0_f64 * t21752 - t2384 * t4402 / 16.0_f64;
    t21756
}
