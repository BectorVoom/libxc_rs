//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1936/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1936(t7810: f64, t999: f64, t7145: f64, t1976: f64, t4746: f64, t1096: f64, t7821: f64, t7160: f64, t4772: f64, t1982: f64, t4930: f64, t1000: f64, t1647: f64, t1652: f64, t1696: f64, t1978: f64, t1986: f64, t25634: f64, t25658: f64, t25692: f64, t25695: f64, t4743: f64, t4764: f64, t4773: f64, t4941: f64, t5016: f64, t7102: f64, t7137: f64, t7140: f64, t7151: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27556 = t7810 * t999;
    let t27557 = t7145 * t27556;
    let t27568 = t4746 * t1976;
    let t27575 = t7821 * t1096;
    let t27576 = t7160 * t27575;
    let t27579 = t1976 * t4772;
    let t27580 = t7145 * t27579;
    let t27587 = t1982 * t4930;
    let t27592 = 0.65854491829355115987e0_f64 * t7102 * t4764 + 0.8673628188205199462e0_f64 * t7151 * t27557 + 0.65854491829355115987e0_f64 * t4743 * t1978 + 0.65854491829355115987e0_f64 * t1647 * t7137 - 0.65854491829355115987e0_f64 * t25634 * t1696 - 0.65854491829355115987e0_f64 * t7102 * t4773 - 0.65854491829355115987e0_f64 * t27568 * t1000 - 0.65854491829355115987e0_f64 * t25695 * t1652 + 0.65854491829355115987e0_f64 * t7102 * t4941 - 0.17347256376410398924e1_f64 * t7151 * t27576 + 0.8673628188205199462e0_f64 * t7151 * t27580 - 0.65854491829355115987e0_f64 * t7140 * t5016 - 0.65854491829355115987e0_f64 * t25692 * t1652 - 0.4336814094102599731e0_f64 * t27587 * t1986 - 0.65854491829355115987e0_f64 * t25658 * t1696;
    (t27556, t27557, t27568, t27575, t27576, t27579, t27580, t27587, t27592)
}
