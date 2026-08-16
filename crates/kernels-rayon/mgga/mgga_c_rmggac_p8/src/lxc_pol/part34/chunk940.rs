//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 940/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk940(t76741: f64, t7720: f64, t73851: f64, t73854: f64, t73865: f64, t73871: f64, t73873: f64, t73875: f64, t73877: f64, t73879: f64, t73857: f64, t73862: f64, t76713: f64, t76718: f64, t76723: f64, t76728: f64, t76733: f64, t76738: f64) -> f64 {
    let t76742 = t7720 * t76741;
    let t76743 = 0.42564599893297839398e-5_f64 * t76742;
    let t76744 = 0.2627895913935205078e-5_f64 * t73851;
    let t76745 = 0.2627895913935205078e-5_f64 * t73854;
    let t76748 = 0.19709219354514038085e-5_f64 * t73865;
    let t76749 = 0.64054962902170623776e-5_f64 * t73871;
    let t76750 = 0.85129199786595678799e-5_f64 * t73873;
    let t76751 = 0.2553875993597870364e-4_f64 * t73875;
    let t76752 = 0.2553875993597870364e-4_f64 * t73877;
    let t76753 = 0.1702583995731913576e-4_f64 * t73879;
    let t76754 = -t76713 + t76718 - t76723 + t76728 - t76733 - t76738 + t76743 + t76744 - t76745 - 0.87596530464506835935e-6_f64 * t73857 + 0.87596530464506835935e-6_f64 * t73862 - t76748 - t76749 + t76750 - t76751 + t76752 + t76753;
    t76754
}
