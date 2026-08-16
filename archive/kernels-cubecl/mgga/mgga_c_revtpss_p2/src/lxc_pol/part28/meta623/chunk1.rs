//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2208/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2208<F: Float>(t15731: F, t7122: F, t15938: F, t16017: F, t16070: F, t16144: F, t16196: F, t16210: F, t1671: F, t1675: F, t25522: F, t27498: F, t4912: F, t7132: F, t93541: F, t93561: F, t93649: F, t93670: F, t99983: F, t99985: F) -> F {
    let t100002 = t7122 * t15731;
    let t100004 = F::cast_from(0.3811023832717309953e-3_f64) * t93541 + F::cast_from(0.28582678745379824648e-3_f64) * t93561 * t1675 + t99983 + F::cast_from(0.42874018118069736972e-3_f64) * t99985 * t16070 + F::cast_from(0.45732285992607719436e-2_f64) * t93670 * t4912 - F::cast_from(0.85748036236139473944e-3_f64) * t27498 * t16017 + F::cast_from(0.57165357490759649296e-3_f64) * t25522 * t16144 - F::cast_from(0.45732285992607719436e-2_f64) * t93649 * t1671 + F::cast_from(0.17149607247227894789e-2_f64) * t7132 * t15938 - F::cast_from(0.57165357490759649296e-3_f64) * t7132 * t16196 + F::cast_from(0.1270341277572436651e-2_f64) * t7132 * t16210 - F::cast_from(0.95275595817932748827e-4_f64) * t100002;
    t100004
}
