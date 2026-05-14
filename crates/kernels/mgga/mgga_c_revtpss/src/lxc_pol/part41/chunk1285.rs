//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1285/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1285<F: Float>(t5854: F, t607: F, t10355: F, t5819: F, t606: F, t4186: F, t4201: F, t2275: F, t5825: F, t18281: F, t48: F, t10368: F, t4210: F, t2282: F, t60: F, t10379: F, t1480: F, t4211: F, t4214: F, t44: F, t56: F, t5835: F, t5838: F, t5843: F, t614: F, t620: F) -> (F, F) {
    let t21727 = t607 * t5854;
    let t21732 = t10355 * t5819;
    let t21733 = t21732 * t606;
    let t21736 = t4201 * t4186;
    let t21741 = t2275 * t5825;
    let t21742 = t21741 * t606;
    let t21745 = t48 * t18281;
    let t21754 = t10368 * t5819;
    let t21755 = t21754 * t606;
    let t21758 = t4210 * t4186;
    let t21761 = t2282 * t5825;
    let t21762 = t21761 * t606;
    let t21765 = t60 * t18281;
    let t21768 = -20.0 / 27.0 * t614 * t5835 - 5.0 / 108.0 * t44 * t21733 + 5.0 / 9.0 * t44 * t21736 - 20.0 / 9.0 * t614 * t5838 + 5.0 / 18.0 * t44 * t21742 + 5.0 / 6.0 * t44 * t21745 - 220.0 / 27.0 * t5843 * t620 - 40.0 / 27.0 * t1480 * t4211 + 40.0 / 9.0 * t1480 * t4214 + 5.0 / 108.0 * t56 * t21755 + 5.0 / 9.0 * t56 * t21758 + 5.0 / 18.0 * t56 * t21762 - 5.0 / 6.0 * t56 * t21765 + t10379;
    (t21727, t21768)
}
