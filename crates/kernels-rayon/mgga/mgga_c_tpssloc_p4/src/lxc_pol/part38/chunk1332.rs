//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1332/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1332(t1395: f64, t8256: f64, t1404: f64, t8240: f64, t2186: f64, t5381: f64, t30217: f64, t580: f64, t110014: f64, t110018: f64, t110489: f64, t110655: f64, t110872: f64, t110877: f64, t1398: f64, t16507: f64, t1852: f64, t1858: f64, t2193: f64, t29979: f64, t3: f64, t30017: f64, t3932: f64) -> f64 {
    let t110882 = 2.0_f64 * t1395 * t8256;
    let t110884 = 2.0_f64 * t8240 * t1404;
    let t110886 = 2.0_f64 * t2186 * t5381;
    let t110888 = 2.0_f64 * t30217 * t580;
    let t110893 = t110489 + t1852 * t30017 + t29979 * t1858 + t1398 * (t110655 + t110877) + t16507 * t2193 + t110882 + t110884 + t110886 + t110888 + t3 * t110872 * t580 + t110018 + t3932 * t8256 + 2.0_f64 * t110014;
    t110893
}
