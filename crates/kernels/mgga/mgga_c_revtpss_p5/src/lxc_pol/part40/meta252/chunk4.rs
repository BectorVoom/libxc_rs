//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 946/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk946<F: Float>(t5674: F, t5675: F, t5673: F, t1388: F, t1410: F, t3931: F, t3956: F, t4022: F, t4064: F, t5606: F, t5611: F, t5614: F, t5619: F, t5623: F, t5625: F, t5629: F, t5661: F, t5666: F, t5671: F) -> (F, F) {
    let t5676 = t5674 * t5675;
    let t5677 = t5673 * t5676;
    let t5680 = t3956 + F::cast_from(0.40015750243531754507e-2_f64) * t5606 + F::cast_from(0.71456696863449561619e-5_f64) * t5611 - F::cast_from(0.85748036236139473944e-3_f64) * t1410 * t5614 - t4064 + F::cast_from(0.28582678745379824648e-4_f64) * t5619 - F::cast_from(0.50820002809285328225e-4_f64) * t5623 + F::cast_from(0.10003937560882938627e-2_f64) * t5625 + F::cast_from(0.42874018118069736972e-2_f64) * t1410 * t5629 - F::cast_from(0.21437009059034868486e-3_f64) * t1388 * t5661 - F::cast_from(0.12705000702321332056e-4_f64) * t5666 + F::cast_from(0.10003937560882938627e-2_f64) * t3931 - F::cast_from(0.12705000702321332056e-4_f64) * t4022 + F::cast_from(0.42874018118069736972e-3_f64) * t5671 * t5677;
    (t5677, t5680)
}
