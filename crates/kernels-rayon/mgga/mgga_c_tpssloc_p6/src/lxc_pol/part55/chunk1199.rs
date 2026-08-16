//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1199/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1199(t118837: f64, t118838: f64, t118841: f64, t118847: f64, t118850: f64, t118851: f64, t118859: f64, t118871: f64, t118874: f64, t118877: f64, t1911: f64, t25329: f64, t259: f64, t2597: f64, t2718: f64, t30647: f64, t30651: f64, t32800: f64, t32849: f64, t4268: f64, t4300: f64, t798: f64, t8362: f64, t855: f64) -> f64 {
    let t118878 = 4.0_f64 * t1911 * t25329 * t2718 * t855 + 2.0_f64 * t2718 * t4300 * t8362 * t855 + t259 * t32849 * t798 + 4.0_f64 * t2597 * t32800 + 2.0_f64 * t30647 * t4268 - 6.0_f64 * t30651 * t4268 - t118837 - t118838 - t118841 + t118847 - t118850 - t118851 + t118859 - t118871 - t118874 + t118877;
    t118878
}
