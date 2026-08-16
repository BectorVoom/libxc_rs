//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1042/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1042(t38775: f64, t38818: f64, t1356: f64, t27075: f64, t37218: f64, t38752: f64, t38755: f64, t38757: f64, t38760: f64, t38764: f64, t38780: f64, t38784: f64, t38793: f64, t38796: f64, t38799: f64, t38802: f64, t38807: f64, t38813: f64, t8041: f64) -> f64 {
    let t42740 = 0.36366215538993788974e-1_f64 * t38775;
    let t42749 = 0.1440846329149835838e-2_f64 * t38818;
    let t42750 = -0.72042316457491791901e-3_f64 * t38752 - 0.72042316457491791901e-3_f64 * t38755 - 0.30487649791575028312e-3_f64 * t38757 - 0.72042316457491791901e-3_f64 * t38760 - 0.72042316457491791901e-3_f64 * t38764 - 0.11974241701863808564e0_f64 * t1356 * t8041 * t27075 + t42740 + 0.85129199786595678799e-5_f64 * t38780 + 0.20001418546446583936e0_f64 * t38784 + 0.71845450211182851384e0_f64 * t38793 - 0.35922725105591425692e0_f64 * t38796 - 0.14369090042236570277e1_f64 * t38799 - 0.35922725105591425692e0_f64 * t38802 - t37218 + 0.40911992481368012596e-1_f64 * t38807 + 0.2993560425465952141e-1_f64 * t38813 + t42749;
    t42750
}
