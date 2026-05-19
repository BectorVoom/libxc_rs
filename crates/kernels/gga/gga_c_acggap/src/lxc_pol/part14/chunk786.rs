//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 786/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk786<F: Float>(t2031: F, t507: F, t2030: F, t2061: F, t2060: F, t2314: F, t7447: F, t7648: F, t7650: F, t7652: F, t7654: F, t7672: F, t8801: F, t8804: F, t8808: F, t8811: F, t8814: F, t8818: F, t8821: F) -> (F, F, F, F) {
    let t8823 = t507 * t2031;
    let t8824 = t2030 * t8823;
    let t8826 = t507 * t2061;
    let t8827 = t2060 * t8826;
    let t8829 = t7447 * t2314;
    let t8834 = F::cast_from(0.42874018118069736972e-3_f64) * t7648 + t8801 / F::new(128.0) + t8804 / F::new(192.0) + t8808 / F::new(16.0) + t8811 / F::new(48.0) + F::cast_from(0.114609375e-1_f64) * t8814 + F::cast_from(0.114609375e-1_f64) * t8818 + F::new(0.7640625e-2) * t8821 + F::cast_from(0.114609375e-1_f64) * t8824 + F::new(0.7640625e-2) * t8827 - F::cast_from(0.420234375e-1_f64) * t8829 + F::cast_from(0.17149607247227894789e-2_f64) * t7650 - F::cast_from(0.85748036236139473944e-3_f64) * t7652 + F::cast_from(0.85748036236139473944e-3_f64) * t7654 + t7672;
    (t8823, t8826, t8829, t8834)
}
