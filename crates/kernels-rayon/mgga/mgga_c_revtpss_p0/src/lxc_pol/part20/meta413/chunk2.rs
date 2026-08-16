//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1527/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1527(t11631: f64, t42871: f64, t3144: f64, t42860: f64, t42866: f64, t3154: f64, t2434: f64, t246: f64, t1041: f64, t1046: f64, t10326: f64, t1042: f64, t1047: f64, t11252: f64, t12021: f64, t3097: f64, t3127: f64, t3136: f64, t3150: f64, t3155: f64, t42386: f64, t42870: f64, t42962: f64, t42965: f64, t42967: f64, t42970: f64, t42973: f64, t42977: f64, t4872: f64, t999: f64) -> (f64, f64) {
    let t42978 = t42871 * t11631;
    let t42984 = t42860 * t3144 * t42866;
    let t42985 = t42871 * t3154;
    let t42994 = t246 * t2434;
    let t42996 = t1041 * t42994 * t1046;
    let t42998 = -0.57165357490759649296e-3_f64 * t3127 * t1042 * t4872 * t10326 * t999 + 0.12862205435420921092e-2_f64 * t12021 * t3136 + 0.22866142996303859718e-2_f64 * t42962 - 0.22866142996303859718e-2_f64 * t42965 - 0.18292914397043087775e-1_f64 * t42967 * t3097 - 0.13719685797782315831e-1_f64 * t42970 * t1047 - 0.51448821741683684368e-2_f64 * t42973 * t11252 - 0.77173232612525526552e-2_f64 * t42977 * t1042 * t42870 * t42978 + 0.30011812682648815881e-2_f64 * t42984 * t1042 * t42870 * t42985 + 0.12862205435420921092e-2_f64 * t3150 * t1042 * t42386 * t3155 + 0.2540682555144873302e-3_f64 * t42996;
    (t42994, t42998)
}
