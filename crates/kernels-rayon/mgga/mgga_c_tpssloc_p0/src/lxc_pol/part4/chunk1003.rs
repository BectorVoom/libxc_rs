//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1003/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1003(t5544: f64, t845: f64, t776: f64, t16662: f64, t824: f64, t1504: f64, t1506: f64, t16723: f64, t16729: f64, t16737: f64, t16740: f64, t228: f64, t230: f64, t4219: f64, t4225: f64, t4227: f64, t4230: f64, t5601: f64, t5605: f64, t5608: f64, t822: f64, t825: f64) -> f64 {
    let t16745 = t845 * t5544;
    let t16746 = t16745 * t776;
    let t16749 = t824 * t16662;
    let t16752 = 6.0_f64 * t1504 * t4230 + 6.0_f64 * t1506 * t4219 - t16723 * t230 - 24.0_f64 * t16729 * t4227 + 60.0_f64 * t16737 * t4225 - 24.0_f64 * t16740 * t4225 - 12.0_f64 * t16746 * t4225 + 3.0_f64 * t16749 * t228 + 3.0_f64 * t5601 * t825 - 12.0_f64 * t5605 * t822 + 3.0_f64 * t5608 * t822;
    t16752
}
