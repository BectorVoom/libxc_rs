//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 986/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk986(t76145: f64, t14512: f64, t8568: f64, t2344: f64, t71876: f64, t14516: f64, t8526: f64, t2329: f64, t71882: f64, t76148: f64, t76151: f64, t76154: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77837 = 0.2993560425465952141e-1_f64 * t76145;
    let t77838 = t14512 * t8568;
    let t77839 = 0.68186654135613354322e-2_f64 * t77838;
    let t77840 = t71876 * t2344;
    let t77841 = 0.10227998120342003148e-1_f64 * t77840;
    let t77842 = t14516 * t8526;
    let t77843 = 0.10227998120342003148e-1_f64 * t77842;
    let t77844 = t71882 * t2329;
    let t77845 = 0.13637330827122670864e-1_f64 * t77844;
    let t77846 = 0.40911992481368012596e-1_f64 * t76148;
    let t77848 = 0.40911992481368012595e-1_f64 * t76151;
    let t77849 = 0.5454932330849068346e-1_f64 * t76154;
    (t77837, t77839, t77841, t77843, t77845, t77846, t77848, t77849)
}
