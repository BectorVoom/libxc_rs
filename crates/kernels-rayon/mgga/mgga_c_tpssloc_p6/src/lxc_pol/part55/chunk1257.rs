//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1257/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1257(t75795: f64, t8319: f64, t1395: f64, t1458: f64, t25994: f64, t7266: f64, t652: f64, t6534: f64, t8103: f64, t26168: f64, t8690: f64, t33746: f64, t6880: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120848 = 27.0_f64 * t75795 * t8319;
    let t120849 = t1395 * t1458;
    let t120851 = 27.0_f64 * t120849 * t8319;
    let t122875 = t7266 * t25994;
    let t122897 = t652 * t8103 * t6534;
    let t122910 = t8690 * t26168;
    let t122914 = t33746 * t6880;
    (t120848, t120851, t122875, t122897, t122910, t122914)
}
