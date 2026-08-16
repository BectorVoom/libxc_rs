//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1144/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1144(t23083: f64, t30706: f64, t23094: f64, t30703: f64, t23103: f64, t794: f64, t8339: f64, t30719: f64, t808: f64, t8344: f64, t226: f64, t235: f64, t2690: f64) -> (f64, f64, f64, f64, f64) {
    let t112829 = t23083 * t30706;
    let t112834 = t23094 * t30703;
    let t112835 = 0.21083550404717759669e-2_f64 * t112834;
    let t112840 = t23103 * t794 * t8339;
    let t112841 = 0.6728792682356731809e-4_f64 * t112840;
    let t112846 = t808 * t30719 * t8344;
    let t112850 = t226 * t235 * t2690 * t8344;
    (t112829, t112835, t112841, t112846, t112850)
}
