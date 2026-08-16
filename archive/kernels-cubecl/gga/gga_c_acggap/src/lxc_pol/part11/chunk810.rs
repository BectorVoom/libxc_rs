//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 810/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk810<F: Float>(t1165: F, t604: F, t8791: F, t7413: F, t7615: F, t7617: F, t7622: F, t7624: F, t7628: F, t7632: F, t7639: F, t7641: F, t7644: F, t8772: F, t8776: F, t8780: F, t8784: F, t8788: F) -> (F, F) {
    let t8793 = t1165 * t604 * t8791;
    let t8794 = t7413 * t8793;
    let t8797 = F::cast_from(0.80031500487063509016e-2_f64) * t7615 - F::cast_from(0.40015750243531754508e-2_f64) * t7617 + F::cast_from(0.40015750243531754508e-2_f64) * t7622 - F::cast_from(0.17149607247227894789e-2_f64) * t7624 + F::cast_from(0.85748036236139473944e-3_f64) * t7628 + t7632 + t7639 - t7641 - F::cast_from(0.7640625e-2_f64) * t8772 + F::cast_from(0.53592522647587171215e-3_f64) * t8776 + F::cast_from(0.21437009059034868486e-3_f64) * t8780 - F::cast_from(0.7862023072401038017e-3_f64) * t8784 - F::cast_from(0.47172138434406228102e-3_f64) * t8788 - F::cast_from(0.31448092289604152068e-3_f64) * t8794 + F::cast_from(0.53592522647587171215e-3_f64) * t7644;
    (t8793, t8797)
}
