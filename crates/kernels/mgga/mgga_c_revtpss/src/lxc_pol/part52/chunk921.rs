//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 921/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk921<F: Float>(t7810: F, t999: F, t7145: F, t1976: F, t4746: F, t1096: F, t7821: F, t7160: F, t4772: F, t1982: F, t4930: F, t1000: F, t1647: F, t1652: F, t1696: F, t1978: F, t1986: F, t25634: F, t25658: F, t25692: F, t25695: F, t4743: F, t4764: F, t4773: F, t4941: F, t5016: F, t7102: F, t7137: F, t7140: F, t7151: F) -> F {
    let t27556 = t7810 * t999;
    let t27557 = t7145 * t27556;
    let t27568 = t4746 * t1976;
    let t27575 = t7821 * t1096;
    let t27576 = t7160 * t27575;
    let t27579 = t1976 * t4772;
    let t27580 = t7145 * t27579;
    let t27587 = t1982 * t4930;
    let t27592 = F::cast_from(0.65854491829355115987e0_f64) * t7102 * t4764 + F::cast_from(0.8673628188205199462e0_f64) * t7151 * t27557 + F::cast_from(0.65854491829355115987e0_f64) * t4743 * t1978 + F::cast_from(0.65854491829355115987e0_f64) * t1647 * t7137 - F::cast_from(0.65854491829355115987e0_f64) * t25634 * t1696 - F::cast_from(0.65854491829355115987e0_f64) * t7102 * t4773 - F::cast_from(0.65854491829355115987e0_f64) * t27568 * t1000 - F::cast_from(0.65854491829355115987e0_f64) * t25695 * t1652 + F::cast_from(0.65854491829355115987e0_f64) * t7102 * t4941 - F::cast_from(0.17347256376410398924e1_f64) * t7151 * t27576 + F::cast_from(0.8673628188205199462e0_f64) * t7151 * t27580 - F::cast_from(0.65854491829355115987e0_f64) * t7140 * t5016 - F::cast_from(0.65854491829355115987e0_f64) * t25692 * t1652 - F::cast_from(0.4336814094102599731e0_f64) * t27587 * t1986 - F::cast_from(0.65854491829355115987e0_f64) * t25658 * t1696;
    t27592
}
