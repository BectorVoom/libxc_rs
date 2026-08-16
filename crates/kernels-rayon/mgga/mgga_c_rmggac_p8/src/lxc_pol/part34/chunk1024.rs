//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1024/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1024(t75725: f64, t69976: f64, t69983: f64, t71582: f64, t75689: f64, t75692: f64, t75695: f64, t75700: f64, t75703: f64, t75718: f64, t77782: f64, t77785: f64, t77788: f64, t77791: f64, t77792: f64, t77793: f64, t77794: f64) -> f64 {
    let t77795 = 0.44903406381989282115e-1_f64 * t75725;
    let t77796 = 0.54549323308490683461e-1_f64 * t69976;
    let t77797 = 0.72732431077987577948e-1_f64 * t69983;
    let t77798 = -0.81756761766873046877e-6_f64 * t75689 + 0.52557918278704101564e-6_f64 * t75692 + 0.87596530464506835935e-6_f64 * t75695 - 0.87596530464506835935e-6_f64 * t75700 + 0.17519306092901367188e-6_f64 * t75703 - t77782 - t77785 + t77788 + t77791 - t75718 - t77792 + t77793 + t77794 - t77795 + t77796 - t77797 + t71582;
    t77798
}
