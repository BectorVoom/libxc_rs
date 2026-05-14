//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 717/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk717<F: Float>(t1562: F, t7561: F, t1466: F, t7822: F, t1470: F, t8632: F, t8635: F, t8638: F, t8640: F, t8642: F, t8644: F, t8646: F, t8650: F, t8654: F, t8658: F, t8662: F, t8666: F) -> (F,) {
    let t8668 = t7561 * t1562;
    let t8670 = t7822 * t1466;
    let t8672 = t7822 * t1470;
    let t8674 = 0.7640625e-2 * t8632 + t8635 / 32.0 + t8638 / 128.0 + 0.17149607247227894789e-2 * t8640 - 0.85748036236139473944e-3 * t8642 + 0.85748036236139473944e-3 * t8644 - 0.42874018118069736972e-3 * t8646 + 0.53592522647587171215e-3 * t8650 + 0.32155513588552302729e-2 * t8654 - 0.47172138434406228102e-3 * t8658 - 0.23586069217203114051e-2 * t8662 + 0.10482697429868050689e-3 * t8666 + 0.17149607247227894789e-2 * t8668 - 0.85748036236139473944e-3 * t8670 + 0.85748036236139473944e-3 * t8672;
    (t8674,)
}
