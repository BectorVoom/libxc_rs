//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 603/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk603<F: Float>(t1089: F, t175: F, t4555: F, t384: F, t1429: F, t997: F, t1418: F, t1347: F, t1165: F, t1532: F, t3084: F, t1531: F, t3306: F, t3308: F, t3310: F, t3312: F, t3314: F, t3316: F, t418: F, t4518: F, t4524: F, t4528: F, t4532: F, t4535: F, t4538: F, t4542: F, t4547: F, t4552: F) -> (F, F, F) {
    let t4557 = t1089 * t175 * t4555;
    let t4558 = t384 * t4557;
    let t4561 = F::cast_from(0.40015750243531754508e-1_f64) * t997 * t1429;
    let t4563 = F::cast_from(0.16006300097412701803e-1_f64) * t997 * t1418;
    let t4565 = F::cast_from(0.16006300097412701803e-1_f64) * t997 * t1347;
    let t4567 = t1165 * t1532 * t3084;
    let t4570 = -F::cast_from(0.17149607247227894789e-2_f64) * t3306 - F::cast_from(0.34299214494455789578e-2_f64) * t3308 + F::cast_from(0.34299214494455789578e-2_f64) * t3310 + F::cast_from(0.34299214494455789577e-2_f64) * t3312 - F::cast_from(0.17149607247227894789e-2_f64) * t3314 + F::cast_from(0.17149607247227894789e-2_f64) * t3316 - F::cast_from(0.85748036236139473944e-3_f64) * t418 * t4518 + F::cast_from(0.42874018118069736972e-3_f64) * t4524 - F::cast_from(0.85748036236139473944e-3_f64) * t418 * t4528 - t4532 + F::cast_from(0.17149607247227894789e-2_f64) * t418 * t4535 + F::cast_from(0.80031500487063509015e-2_f64) * t4538 - F::cast_from(0.17149607247227894789e-2_f64) * t418 * t4542 - F::cast_from(0.17149607247227894789e-2_f64) * t418 * t4547 + F::cast_from(0.17149607247227894789e-2_f64) * t418 * t4552 + F::cast_from(0.85748036236139473944e-3_f64) * t4558 - t4561 + t4563 - t4565 + F::cast_from(0.42874018118069736972e-3_f64) * t1531 * t4567;
    (t4557, t4567, t4570)
}
