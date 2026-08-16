//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1267/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1267(t30720: f64, t849: f64, t23083: f64, t30706: f64, t23094: f64, t30703: f64, t23103: f64, t794: f64, t8339: f64, t30719: f64, t808: f64, t8344: f64) -> (f64, f64, f64, f64, f64) {
    let t112820 = t30720 * t849;
    let t112821 = 7.0_f64 / 288.0_f64 * t112820;
    let t112829 = t23083 * t30706;
    let t112830 = 0.11304371706359309439e-1_f64 * t112829;
    let t112834 = t23094 * t30703;
    let t112840 = t23103 * t794 * t8339;
    let t112846 = t808 * t30719 * t8344;
    (t112821, t112830, t112834, t112840, t112846)
}
