//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2986/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2986<F: Float>(t1042: F, t1063: F, t11703: F, t16095: F, t16196: F, t3092: F, t3127: F, t3188: F, t43313: F, t4573: F, t4578: F, t4801: F, t54419: F, t54432: F, t54435: F, t54438: F, t54440: F, t54443: F, t54446: F, t54450: F, t906: F) -> F {
    let t54455 = -F::cast_from(0.42874018118069736972e-3_f64) * t3127 * t1042 * t54419 * t906 + F::cast_from(0.85748036236139473944e-3_f64) * t16095 * t3092 * t4578 * t43313 - F::cast_from(0.71456696863449561621e-3_f64) * t16095 * t11703 * t4573 * t43313 - F::cast_from(0.11433071498151929859e-2_f64) * t54432 - F::cast_from(0.57165357490759649295e-3_f64) * t54435 - F::cast_from(0.28582678745379824648e-2_f64) * t54438 + F::cast_from(0.95275595817932748827e-3_f64) * t54440 + F::cast_from(0.47637797908966374414e-3_f64) * t54443 + F::cast_from(0.1270341277572436651e-2_f64) * t54446 - F::cast_from(0.85748036236139473944e-3_f64) * t3188 * t16196 - F::cast_from(0.28582678745379824648e-3_f64) * t1063 * t1042 * t4801 * t54450;
    t54455
}
