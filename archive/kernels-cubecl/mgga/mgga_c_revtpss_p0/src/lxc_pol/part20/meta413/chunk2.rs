//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1527/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1527<F: Float>(t11631: F, t42871: F, t3144: F, t42860: F, t42866: F, t3154: F, t2434: F, t246: F, t1041: F, t1046: F, t10326: F, t1042: F, t1047: F, t11252: F, t12021: F, t3097: F, t3127: F, t3136: F, t3150: F, t3155: F, t42386: F, t42870: F, t42962: F, t42965: F, t42967: F, t42970: F, t42973: F, t42977: F, t4872: F, t999: F) -> (F, F) {
    let t42978 = t42871 * t11631;
    let t42984 = t42860 * t3144 * t42866;
    let t42985 = t42871 * t3154;
    let t42994 = t246 * t2434;
    let t42996 = t1041 * t42994 * t1046;
    let t42998 = -F::cast_from(0.57165357490759649296e-3_f64) * t3127 * t1042 * t4872 * t10326 * t999 + F::cast_from(0.12862205435420921092e-2_f64) * t12021 * t3136 + F::cast_from(0.22866142996303859718e-2_f64) * t42962 - F::cast_from(0.22866142996303859718e-2_f64) * t42965 - F::cast_from(0.18292914397043087775e-1_f64) * t42967 * t3097 - F::cast_from(0.13719685797782315831e-1_f64) * t42970 * t1047 - F::cast_from(0.51448821741683684368e-2_f64) * t42973 * t11252 - F::cast_from(0.77173232612525526552e-2_f64) * t42977 * t1042 * t42870 * t42978 + F::cast_from(0.30011812682648815881e-2_f64) * t42984 * t1042 * t42870 * t42985 + F::cast_from(0.12862205435420921092e-2_f64) * t3150 * t1042 * t42386 * t3155 + F::cast_from(0.2540682555144873302e-3_f64) * t42996;
    (t42994, t42998)
}
