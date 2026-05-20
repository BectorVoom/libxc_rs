//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1187/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1187<F: Float>(t324: F, t4682: F, t1626: F, t964: F, t1634: F, t972: F, t2848: F, t2906: F, t2994: F, t3001: F, t4571: F, t4576: F, t4581: F, t4585: F, t4599: F, t4607: F, t4615: F, t4617: F, t4620: F, t4623: F, t4626: F, t4629: F) -> (F, F, F, F) {
    let t4683 = t4682 * t324;
    let t4685 = t1626 * t964;
    let t4690 = t1634 * t972;
    let t4707 = -F::new(0.1294625e1) * t4599 + F::new(0.258925e1) * t4607 + t2994 + F::cast_from(0.10064166666666666667e0_f64) * t2848 + F::cast_from(0.10064166666666666667e0_f64) * t4571 - F::cast_from(0.20128333333333333333e0_f64) * t4576 + F::new(0.60385e0) * t4581 - F::new(0.301925e0) * t4585 + F::new(0.82524375e-1) * t4615 + F::new(0.16504875e0) * t4617 + t3001 + F::new(0.5519e-1) * t2906 + F::new(0.5519e-1) * t4620 - F::new(0.27595e-1) * t4623 + F::new(0.16557e0) * t4626 - F::new(0.82785e-1) * t4629;
    (t4683, t4685, t4690, t4707)
}
