//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta398 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1376;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1377;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1378;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta398<F: Float>(t18426: F, t4364: F, t4366: F, t2741: F, t5980: F, t4365: F, t4424: F, t837: F, t125: F, t5966: F, t10770: F, t2652: F, t5993: F, t14586: F, t14786: F, t14791: F, t1559: F, t4433: F, t14785: F, t6030: F, t10858: F, t6024: F, t10816: F, t10824: F, t10826: F, t2745: F, t4362: F, t6019: F, t775: F, t10698: F, t828: F, t1544: F, t4343: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t18456, t18459, t18462, t18466, t18471, t18475) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1376::<F>(t18426, t4364, t4366, t2741, t5980, t4365, t4424, t837, t125, t5966, t10770, t2652, t5993);
        let (t18478, t18482, t18489) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1377::<F>(t14586, t14786, t14791, t1559, t4433, t14785, t2652, t6030, t10858, t6024, t10816, t10824, t10826, t18456, t18459, t18462, t18466, t18471, t18475, t2745, t4362);
        let (t18491, t18493, t18495, t18498) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1378::<F>(t2741, t6019, t5966, t775, t10698, t828, t1544, t4343);
    (t18456, t18462, t18466, t18471, t18478, t18482, t18489, t18491, t18493, t18495, t18498)
}
