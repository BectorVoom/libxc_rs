//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1198/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1198(t14869: f64, t2661: f64, t231: f64, t2430: f64, t2747: f64, t4365: f64, t10762: f64, t10783: f64, t10812: f64, t10816: f64, t10900: f64, t14843: f64, t14846: f64, t14850: f64, t14853: f64, t14859: f64, t14864: f64, t14866: f64, t2745: f64, t851: f64) -> f64 {
    let t14871 = 0.28582678745379824648e-4_f64 * t2661 * t14869;
    let t14872 = t231 * t2430;
    let t14874 = t2747 * t4365 * t14872;
    let t14878 = -t10900 * t14843 / 4.0_f64 - 0.30488190661738479625e-3_f64 * t14846 - 0.90357964994909313582e-5_f64 * t10762 + 0.10164000561857065645e-3_f64 * t10783 - 0.76220476654346199061e-4_f64 * t14850 - 0.85748036236139473944e-3_f64 * t851 * t14853 - t14859 + t14864 - 0.80031500487063509016e-2_f64 * t10812 - 0.22675591804667994221e-1_f64 * t14866 - t14871 + 0.85748036236139473944e-3_f64 * t2745 * t14874 - 0.11337795902333997111e-1_f64 * t10816;
    t14878
}
