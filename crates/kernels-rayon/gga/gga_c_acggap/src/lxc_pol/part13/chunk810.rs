//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 810/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk810(t1165: f64, t604: f64, t8791: f64, t7413: f64, t7615: f64, t7617: f64, t7622: f64, t7624: f64, t7628: f64, t7632: f64, t7639: f64, t7641: f64, t7644: f64, t8772: f64, t8776: f64, t8780: f64, t8784: f64, t8788: f64) -> (f64, f64) {
    let t8793 = t1165 * t604 * t8791;
    let t8794 = t7413 * t8793;
    let t8797 = 0.80031500487063509016e-2_f64 * t7615 - 0.40015750243531754508e-2_f64 * t7617 + 0.40015750243531754508e-2_f64 * t7622 - 0.17149607247227894789e-2_f64 * t7624 + 0.85748036236139473944e-3_f64 * t7628 + t7632 + t7639 - t7641 - 0.7640625e-2_f64 * t8772 + 0.53592522647587171215e-3_f64 * t8776 + 0.21437009059034868486e-3_f64 * t8780 - 0.7862023072401038017e-3_f64 * t8784 - 0.47172138434406228102e-3_f64 * t8788 - 0.31448092289604152068e-3_f64 * t8794 + 0.53592522647587171215e-3_f64 * t7644;
    (t8793, t8797)
}
