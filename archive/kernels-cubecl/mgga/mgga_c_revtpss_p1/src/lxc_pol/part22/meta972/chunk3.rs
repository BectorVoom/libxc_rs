//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3255/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3255<F: Float>(t6016: F, t836: F, t10811: F, t18482: F, t5977: F, t14749: F, t14785: F, t14791: F, t1559: F, t2745: F, t2749: F, t50518: F, t50522: F, t50524: F, t50526: F, t50529: F, t50531: F, t50540: F) -> (F, F, F) {
    let t61749 = t6016 * t836;
    let t61754 = t10811 * t18482;
    let t61756 = t5977 * t836;
    let t61772 = F::cast_from(0.17149607247227894789e-2_f64) * t2745 * t14791 * t61749 * t2749 + F::cast_from(0.80031500487063509016e-1_f64) * t61754 + F::cast_from(0.17149607247227894789e-2_f64) * t2745 * t14791 * t61756 * t2749 - F::cast_from(0.17149607247227894789e-1_f64) * t2745 * t14785 * t1559 * t14749 + F::cast_from(0.11433071498151929859e-3_f64) * t50518 - F::cast_from(0.85748036236139473944e-4_f64) * t50522 + F::cast_from(0.45351183609335988442e0_f64) * t50524 - F::cast_from(0.80031500487063509016e-1_f64) * t50526 - F::cast_from(0.40015750243531754508e-1_f64) * t50529 - F::cast_from(0.10841600599314203355e-1_f64) * t50531 - F::cast_from(0.30492001685571196935e-2_f64) * t50540;
    (t61749, t61756, t61772)
}
