//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1103/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1103<F: Float>(t221: F, t2675: F, t5962: F, t2674: F, t10756: F, t10758: F, t10762: F, t14836: F, t14837: F, t14839: F, t14846: F, t14850: F, t14859: F, t14864: F, t18394: F, t18398: F, t799: F, t851: F) -> (F,) {
    let t18402 = t2675 * t221 * t5962;
    let t18403 = t2674 * t18402;
    let t18405 = -t14836 + 0.80031500487063509015e-2 * t14837 + 0.10841600599314203355e-2 * t14839 - t10756 - t10758 - 0.60976381323476959249e-3 * t14846 - 0.45178982497454656791e-5 * t10762 - 0.15244095330869239812e-3 * t14850 - t14859 + t14864 - t799 * t18394 / 48.0 - 0.85748036236139473944e-3 * t851 * t18398 - 0.50820002809285328225e-4 * t18403;
    (t18405,)
}
