//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1244/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1244<F: Float>(t1020: F, t11284: F, t11286: F, t11288: F, t1133: F, t1135: F, t1137: F, t1310: F, t1312: F, t2410: F, t3534: F, t3538: F, t3542: F, t3749: F, t3753: F, t3757: F, t3761: F, t3765: F, t8438: F) -> F {
    let t41971 = -F::cast_from(0.3831420472412e2_f64) * t11284 * t1020 - F::cast_from(0.7662840944824e2_f64) * t3534 * t2410 - F::cast_from(0.3831420472412e2_f64) * t1133 * t8438 - F::cast_from(0.3831420472412e2_f64) * t3757 * t1310 + F::cast_from(0.1550653405116e2_f64) * t11286 * t1020 + F::cast_from(0.3101306810232e2_f64) * t3538 * t2410 + F::cast_from(0.1550653405116e2_f64) * t1135 * t8438 + F::cast_from(0.1550653405116e2_f64) * t3761 * t1310 - F::cast_from(0.2177652951264e1_f64) * t11288 * t1020 - F::cast_from(0.4355305902528e1_f64) * t3542 * t2410 - F::cast_from(0.2177652951264e1_f64) * t1137 * t8438 - F::cast_from(0.2177652951264e1_f64) * t3765 * t1310 + F::cast_from(0.734774460522e2_f64) * t3749 * t1312 - F::cast_from(0.11494261417236e3_f64) * t3753 * t1312 + F::cast_from(0.6202613620464e2_f64) * t3757 * t1312;
    t41971
}
