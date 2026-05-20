//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2148/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2148<F: Float>(t19785: F, t25517: F, t100132: F, t16509: F, t16584: F, t19622: F, t19636: F, t19726: F, t19778: F, t19782: F, t20079: F, t27492: F, t27493: F, t4896: F, t4902: F, t6268: F, t93597: F, t93658: F, t93667: F) -> F {
    let t106960 = t25517 * t19785;
    let t106968 = -F::cast_from(0.30488190661738479625e-2_f64) * t93597 * t6268 + F::cast_from(0.17149607247227894789e-2_f64) * t93667 * t19622 + F::cast_from(0.57165357490759649296e-3_f64) * t27493 * t19726 + F::cast_from(0.57165357490759649296e-3_f64) * t25517 * t19778 + F::cast_from(0.28582678745379824648e-3_f64) * t25517 * t20079 - F::cast_from(0.17149607247227894789e-2_f64) * t93658 * t19636 + F::cast_from(0.47637797908966374413e-3_f64) * t25517 * t19782 + F::cast_from(0.38110238327173099531e-3_f64) * t106960 + t100132 + F::cast_from(0.17149607247227894789e-2_f64) * t16509 * t27492 * t4896 - F::cast_from(0.85748036236139473944e-3_f64) * t16584 * t27492 * t4902;
    t106968
}
