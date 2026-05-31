//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 973/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk973<F: Float>(t2105: F, t7682: F, t1120: F, t2057: F, t2104: F, t276: F, t2895: F, t2899: F, t2922: F, t2933: F, t5646: F, t5661: F, t5666: F, t5984: F, t735: F, t7621: F, t7630: F, t7632: F, t7639: F, t7642: F, t7650: F, t7655: F, t7660: F, t7664: F, t7668: F, t7673: F, t7678: F) -> (F, F) {
    let t7683 = t2105 * t7682;
    let t7686 = t7621 / F::cast_from(432.0_f64) - F::cast_from(11.0_f64) / F::cast_from(108.0_f64) * t2057 * t1120 + t735 * t2895 / F::cast_from(18.0_f64) - t7630 - t276 * t7632 / F::cast_from(96.0_f64) + F::cast_from(0.45732285992607719436e-2_f64) * t5984 * t2933 - t7639 + F::cast_from(0.12862205435420921092e-2_f64) * t2104 * t7642 - t5646 / F::cast_from(288.0_f64) + t5661 / F::cast_from(54.0_f64) + t5666 / F::cast_from(144.0_f64) - F::cast_from(0.42874018118069736972e-3_f64) * t2104 * t7650 - F::cast_from(0.42874018118069736972e-3_f64) * t2922 * t7655 - F::cast_from(0.21437009059034868486e-3_f64) * t2922 * t7660 + F::cast_from(0.21437009059034868486e-3_f64) * t7664 * t7668 - F::cast_from(0.85748036236139473944e-3_f64) * t2104 * t7673 - F::cast_from(0.42874018118069736972e-3_f64) * t2104 * t7678 - F::cast_from(0.85748036236139473944e-3_f64) * t2899 * t7683;
    (t7683, t7686)
}
