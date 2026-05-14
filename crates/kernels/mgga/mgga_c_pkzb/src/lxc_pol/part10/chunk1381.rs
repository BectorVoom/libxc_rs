//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1381/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1381<F: Float>(t10012: F, t10019: F, t1197: F, t1209: F, t18706: F, t18866: F, t2257: F, t2258: F, t2273: F, t22767: F, t2279: F, t2296: F, t2297: F, t2312: F, t3106: F, t3780: F, t3792: F, t3793: F, t3796: F, t3820: F, t3823: F, t6288: F, t6290: F, t6313: F, t6323: F, t8067: F, t8098: F, t8120: F, t8142: F, t8154: F, t9992: F) -> (F,) {
    let t27666 = 0.64327917994770140268e2 * t8120 * t8142 + 0.4138081033541872024e4 * t22767 * t8154 + 6.0 * t2279 * t3780 * t2273 + 0.11579025239058625248e4 * t6288 * t3796 * t2258 - 4.0 * t2257 * t1197 * t8067 - 0.19298375398431042081e3 * t6313 * t3796 * t2273 - 0.24828486201251232145e5 * t18706 * t10019 * t2258 - 2.0 * t2257 * t3793 * t2273 - 0.19298375398431042081e3 * t6313 * t10012 * t2258 + 0.32163958997385070134e2 * t2279 * t10012 * t2273 + 0.2069040516770936012e4 * t6288 * t3792 * t6290 * t2258 + 0.64327917994770140268e2 * t2279 * t3106 * t8067 + 0.2069040516770936012e4 * t6288 * t10019 * t2273 - 24.0 * t6313 * t3780 * t2258 + 6.0 * t2279 * t3793 * t2258 - 0.23392894490538584828e1 * t2296 * t1209 * t8098 - 0.10389515463408878255e3 * t6323 * t3823 * t2312 - 0.12304822629859687989e5 * t18866 * t9992 * t2297 - 0.11696447245269292414e1 * t2296 * t3820 * t2312;
    (t27666,)
}
