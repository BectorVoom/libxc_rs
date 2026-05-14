//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1062/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1062<F: Float>(t10850: F, t18432: F, t5962: F, t775: F, t2477: F, t828: F, t14718: F, t6035: F, t2662: F, t2661: F, t125: F, t6016: F, t2747: F, t2749: F, t18426: F, t14866: F, t14871: F, t18411: F, t18416: F, t18420: F, t18424: F, t18428: F, t2745: F, t4362: F, t851: F) -> (F, F) {
    let t18433 = t10850 * t18432;
    let t18435 = t5962 * t775;
    let t18437 = t2477 * t828 * t18435;
    let t18440 = t14718 * t6035;
    let t18441 = t2662 * t18440;
    let t18442 = t2661 * t18441;
    let t18444 = t125 * t6016;
    let t18446 = t2747 * t18444 * t2749;
    let t18451 = t2747 * t18426 * t2749;
    let t18454 = 0.71456696863449561619e-5 * t18411 - 0.14291339372689912324e-4 * t18416 + 0.71456696863449561619e-5 * t18420 + 0.25410001404642664113e-3 * t18424 - 0.17149607247227894789e-2 * t4362 * t18428 + 0.25410001404642664113e-4 * t18433 + 0.42874018118069736972e-2 * t851 * t18437 - 0.57165357490759649296e-4 * t18442 + 0.85748036236139473944e-3 * t2745 * t18446 - 0.45351183609335988442e-1 * t14866 + 0.85748036236139473944e-3 * t2745 * t18451 - t14871;
    (t18444, t18454)
}
