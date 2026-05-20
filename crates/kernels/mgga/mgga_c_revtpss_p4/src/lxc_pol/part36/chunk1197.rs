//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1197/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1197<F: Float>(t1785: F, t8184: F, t2137: F, t6593: F, t467: F, t1782: F, t1791: F, t1797: F, t26824: F, t26870: F, t26877: F, t29010: F, t29062: F, t29072: F, t29077: F, t29086: F, t29089: F, t484: F, t6611: F, t6647: F, t6653: F, t6659: F, t6663: F, t6673: F, t6683: F, t6690: F, t7607: F, t7613: F, t7624: F) -> (F, F, F, F) {
    let t30812 = t1785 * t8184;
    let t30815 = t2137 * t6593;
    let t30816 = t467 * t30815;
    let t30839 = F::cast_from(0.47637797908966374413e-3_f64) * t7624 * t6673 + F::cast_from(0.57165357490759649296e-3_f64) * t29072 - F::cast_from(0.30488190661738479624e-2_f64) * t29077 + F::cast_from(0.85748036236139473944e-3_f64) * t29010 * t1797 - F::cast_from(0.45732285992607719436e-2_f64) * t30812 * t484 + F::cast_from(0.14481890564325777821e-1_f64) * t30816 * t484 - t26877 - F::cast_from(0.57165357490759649296e-3_f64) * t7624 * t6683 - F::cast_from(0.85748036236139473944e-3_f64) * t26870 * t6690 - F::cast_from(0.85748036236139473944e-3_f64) * t29086 * t1791 + F::cast_from(0.85748036236139473944e-3_f64) * t26824 * t6611 + F::cast_from(0.45732285992607719436e-2_f64) * t29062 * t1791 - F::cast_from(0.42874018118069736972e-3_f64) * t7613 * t6647 + t7607 * t6653 / F::new(216.0) + t29089 * t1782 / F::new(54.0) - t7607 * t6659 / F::new(288.0) - t7607 * t6663 / F::new(144.0);
    (t30812, t30815, t30816, t30839)
}
