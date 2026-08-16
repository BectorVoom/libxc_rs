//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 948/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk948(t21760: f64, t21764: f64, t21767: f64, t21771: f64, t21774: f64, t21778: f64, t21781: f64, t21783: f64, t21786: f64, t21789: f64, t21792: f64, t21795: f64, t21802: f64, t21804: f64) -> f64 {
    let t21937 = 0.16504875e0_f64 * t21781 + 0.258925e1_f64 * t21783 + 0.19419375e1_f64 * t21786 - 0.16557e0_f64 * t21789 + 0.49671e0_f64 * t21792 + 0.82785e-1_f64 * t21795 + 0.33547222222222222222e0_f64 * t21760 - 0.12077e1_f64 * t21764 + 0.181155e1_f64 * t21771 + 0.301925e0_f64 * t21778 + 0.36793333333333333333e-1_f64 * t21802 - 0.412621875e-1_f64 * t21804 - 0.60384999999999999999e0_f64 * t21767 + 0.181155e1_f64 * t21774;
    t21937
}
