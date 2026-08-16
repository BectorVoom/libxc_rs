//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 694/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk694(t10809: f64, t1773: f64, t10487: f64, t662: f64, t10441: f64, t5006: f64, t1772: f64, t4983: f64, t5007: f64, t1775: f64, t4989: f64, t4999: f64) -> (f64, f64, f64, f64, f64) {
    let t10810 = t1773 * t10809;
    let t10812 = t662 * t10487;
    let t10813 = t10812 * t10441;
    let t10814 = t5006 * t10813;
    let t10817 = t4983 * t1772;
    let t10820 = t5007 * t10441;
    let t10821 = t1775 * t10820;
    let t10828 = t4989 * t4999;
    (t10810, t10814, t10817, t10821, t10828)
}
