//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1220/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1220<F: Float>(t6400: F, t962: F, t19094: F, t971: F, t1211: F, t6783: F, t10874: F, t10960: F, t1221: F, t1225: F, t1226: F, t15445: F, t1835: F, t18983: F, t18987: F, t18993: F, t18995: F, t18999: F, t3582: F, t5242: F, t5250: F, t6814: F, t6817: F, t6820: F) -> F {
    let t20381 = t6400 * t962;
    let t20392 = t19094 * t971;
    let t20397 = t6783 * t1211;
    let t20400 = -t18983 - t18987 + F::cast_from(0.58482233974552040708e0_f64) * t20381 * t1226 + F::cast_from(0.11696446794910408142e1_f64) * t15445 * t1835 + F::cast_from(0.11696446794910408142e1_f64) * t5242 * t5250 - F::cast_from(0.11696446794910408142e1_f64) * t10960 * t6814 + F::cast_from(0.58482233974552040708e0_f64) * t3582 * t6817 + F::cast_from(0.58482233974552040708e0_f64) * t1225 * t20392 + t18993 - t18995 - t18999 + F::cast_from(0.17315755899375863299e2_f64) * t10874 * t6820 + F::cast_from(1.0_f64) * t20397 * t1221;
    t20400
}
