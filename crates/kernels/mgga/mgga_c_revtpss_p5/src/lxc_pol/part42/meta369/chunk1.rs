//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1197/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1197<F: Float>(t18378: F, t18390: F, t124: F, t800: F, t828: F, t855: F, t221: F, t2675: F, t5962: F, t2674: F, t10756: F, t10758: F, t10762: F, t14836: F, t14837: F, t14839: F, t14846: F, t14850: F, t14859: F, t14864: F, t799: F, t851: F) -> (F, F) {
    let t18392 = t18378 / F::cast_from(2.0_f64) + t18390 / F::cast_from(2.0_f64);
    let t18393 = t124 * t18392;
    let t18394 = t800 * t18393;
    let t18398 = t855 * t828 * t18392;
    let t18402 = t2675 * t221 * t5962;
    let t18403 = t2674 * t18402;
    let t18405 = -t14836 + F::cast_from(0.80031500487063509015e-2_f64) * t14837 + F::cast_from(0.10841600599314203355e-2_f64) * t14839 - t10756 - t10758 - F::cast_from(0.60976381323476959249e-3_f64) * t14846 - F::cast_from(0.45178982497454656791e-5_f64) * t10762 - F::cast_from(0.15244095330869239812e-3_f64) * t14850 - t14859 + t14864 - t799 * t18394 / F::cast_from(48.0_f64) - F::cast_from(0.85748036236139473944e-3_f64) * t851 * t18398 - F::cast_from(0.50820002809285328225e-4_f64) * t18403;
    (t18392, t18405)
}
