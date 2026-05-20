//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1796/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1796<F: Float>(t18444: F, t2747: F, t2749: F, t18426: F, t14866: F, t14871: F, t18411: F, t18416: F, t18420: F, t18424: F, t18428: F, t18433: F, t18437: F, t18442: F, t2745: F, t4362: F, t851: F) -> (F, F, F) {
    let t18446 = t2747 * t18444 * t2749;
    let t18451 = t2747 * t18426 * t2749;
    let t18454 = F::cast_from(0.71456696863449561619e-5_f64) * t18411 - F::cast_from(0.14291339372689912324e-4_f64) * t18416 + F::cast_from(0.71456696863449561619e-5_f64) * t18420 + F::cast_from(0.25410001404642664113e-3_f64) * t18424 - F::cast_from(0.17149607247227894789e-2_f64) * t4362 * t18428 + F::cast_from(0.25410001404642664113e-4_f64) * t18433 + F::cast_from(0.42874018118069736972e-2_f64) * t851 * t18437 - F::cast_from(0.57165357490759649296e-4_f64) * t18442 + F::cast_from(0.85748036236139473944e-3_f64) * t2745 * t18446 - F::cast_from(0.45351183609335988442e-1_f64) * t14866 + F::cast_from(0.85748036236139473944e-3_f64) * t2745 * t18451 - t14871;
    (t18446, t18451, t18454)
}
