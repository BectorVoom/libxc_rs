//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2555/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2555(t1099: f64, t1118: f64, t71558: f64, t71571: f64, t71585: f64, t71597: f64, t71611: f64, t71624: f64, t71636: f64, t71649: f64, t21813: f64, t43964: f64) -> (f64, f64) {
    let t71655 = 1.0_f64 * t1099 * (t71558 + t71571 + t71585 + t71597 + t71611 + t71624 + t71636 + t71649) * t1118;
    let t71657 = 0.51726012919273400301e3_f64 * t43964 * t21813;
    (t71655, t71657)
}
