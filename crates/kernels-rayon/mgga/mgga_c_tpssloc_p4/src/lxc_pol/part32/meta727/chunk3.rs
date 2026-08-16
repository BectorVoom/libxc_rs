//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2356/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2356(t104995: f64, t104996: f64, t1266: f64, t12725: f64, t19456: f64, t27879: f64, t29486: f64, t4028: f64, t574: f64, t7989: f64, t96784: f64, t96786: f64, t96789: f64, t96792: f64, t96796: f64, t96799: f64, t96802: f64, t96805: f64, t96807: f64, t96813: f64, t96815: f64, t96818: f64, t96827: f64, t96829: f64) -> f64 {
    let t105005 = -t96784 - t96786 - t96789 + t96792 + t96796 + t96799 - t96802 + t96805 - t96807 - t96813 - t96815 - t96818 + t96827 - t29486 * t1266 + (t104995 + t104996) * t574 - t96829 - 4.0_f64 * t19456 * t7989 - 4.0_f64 * t4028 * t27879 - 4.0_f64 * t12725 * t7989;
    t105005
}
