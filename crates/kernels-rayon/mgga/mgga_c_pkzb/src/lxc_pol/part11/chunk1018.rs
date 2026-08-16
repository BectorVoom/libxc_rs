//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1018/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1018(t11213: f64, t890: f64, t11180: f64, t6233: f64, t11166: f64, t871: f64, t3740: f64, t8009: f64, t11159: f64, t11164: f64, t11167: f64, t11181: f64, t1197: f64, t1209: f64, t2279: f64, t2318: f64, t3083: f64, t3116: f64, t3780: f64, t3793: f64, t3807: f64, t3820: f64, t3823: f64, t6282: f64, t6288: f64, t6323: f64, t8071: f64, t8107: f64, t8211: f64, t882: f64, t9891: f64, t9964: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11214 = t11213 * t890;
    let t11217 = t11180 * t6233;
    let t11222 = t11166 * t871;
    let t11227 = t11180 * t890;
    let t11231 = 6.0_f64 * t8009 * t3740;
    let t11232 = t11159 - 0.19751673498613801407e-1_f64 * t11164 + 0.2069040516770936012e4_f64 * t6288 * t11167 + 3.0_f64 * t9891 * t1197 + 3.0_f64 * t3083 * t3793 + 0.17544670867903938621e1_f64 * t9964 * t1209 + 0.17544670867903938621e1_f64 * t3116 * t3820 + 0.51947577317044391276e2_f64 * t8107 * t3823 - 0.10389515463408878255e3_f64 * t6323 * t11181 + 0.5848223622634646207e0_f64 * t882 * t11214 + 0.10254018858216406658e4_f64 * t6282 * t11217 - 6.0_f64 * t8211 * t3780 + 6.0_f64 * t2279 * t11222 - 0.35089341735807877242e1_f64 * t8071 * t3807 + 0.35089341735807877242e1_f64 * t2318 * t11227 + t11231;
    (t11214, t11217, t11222, t11227, t11231, t11232)
}
