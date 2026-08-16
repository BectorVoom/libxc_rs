//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1031/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1031(t136: f64, t21801: f64, t11243: f64, t21785: f64, t21760: f64, t21764: f64, t21767: f64, t21771: f64, t21774: f64, t21778: f64, t21781: f64, t21783: f64, t21786: f64, t21789: f64, t21792: f64, t21795: f64) -> (f64, f64, f64) {
    let t21802 = t136 * t21801;
    let t21804 = t11243 * t21785;
    let t21808 = 0.3071625e0_f64 * t21781 + 0.1898925e1_f64 * t21783 + 0.142419375e1_f64 * t21786 - 0.16431333333333333333e0_f64 * t21789 + 0.49293999999999999999e0_f64 * t21792 + 0.82156666666666666667e-1_f64 * t21795 + 0.33218518518518518518e0_f64 * t21760 - 0.11958666666666666667e1_f64 * t21764 + 0.17938e1_f64 * t21771 + 0.29896666666666666667e0_f64 * t21778 + 0.36514074074074074075e-1_f64 * t21802 - 0.76790625e-1_f64 * t21804 - 0.59793333333333333333e0_f64 * t21767 + 0.17938e1_f64 * t21774;
    (t21802, t21804, t21808)
}
