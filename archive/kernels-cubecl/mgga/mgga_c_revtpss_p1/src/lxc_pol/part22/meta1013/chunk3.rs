//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3482/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3482<F: Float>(t1063: F, t19929: F, t3172: F, t19933: F, t19676: F, t3127: F, t1042: F, t11994: F, t12021: F, t15193: F, t15817: F, t15970: F, t15975: F, t16138: F, t19688: F, t19738: F, t19741: F, t19792: F, t19800: F, t3124: F, t3188: F, t4583: F, t4801: F, t4823: F, t4869: F, t6302: F, t65433: F) -> F {
    let t65507 = t1063 * t3172 * t19929;
    let t65510 = t1063 * t3172 * t19933;
    let t65527 = t3127 * t3172 * t19676;
    let t65533 = F::cast_from(0.85748036236139473944e-3_f64) * t15817 * t4869 - F::cast_from(0.57165357490759649296e-3_f64) * t11994 * t19792 - F::cast_from(0.57165357490759649296e-3_f64) * t3127 * t1042 * t16138 * t4583 + F::cast_from(0.11433071498151929859e-2_f64) * t65507 - F::cast_from(0.76220476654346199061e-3_f64) * t65510 - F::cast_from(0.28582678745379824648e-3_f64) * t3127 * t1042 * t4823 * t15193 + F::cast_from(0.47637797908966374414e-3_f64) * t3188 * t19688 + F::cast_from(0.21437009059034868486e-3_f64) * t12021 * t6302 - F::cast_from(0.57165357490759649296e-3_f64) * t1063 * t1042 * t4801 * t65433 + F::cast_from(0.42874018118069736972e-3_f64) * t3124 * t19800 - F::cast_from(0.19055119163586549765e-3_f64) * t65527 + F::cast_from(0.57165357490759649296e-3_f64) * t19738 * t15970 - F::cast_from(0.28582678745379824648e-3_f64) * t19741 * t15975;
    t65533
}
