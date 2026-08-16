//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1625/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1625(t12732: f64, t3153: f64, t12952: f64, t3172: f64, t3711: f64, t1042: f64, t1250: f64, t12787: f64, t12910: f64, t12912: f64, t13081: f64, t17235: f64, t17261: f64, t17729: f64, t17730: f64, t2258: f64, t3362: f64, t3718: f64, t3720: f64, t44205: f64, t44599: f64, t44607: f64, t44609: f64, t44610: f64, t44616: f64, t44618: f64, t44624: f64, t44634: f64, t44637: f64, t5331: f64, t5333: f64, t5340: f64, t5341: f64) -> (f64, f64) {
    let t44639 = t12732 * t3153;
    let t44649 = t3711 * t3172 * t12952;
    let t44657 = -0.85748036236139473944e-3_f64 * t3718 * t3720 * t44599 * t1250 - t44607 - 0.51448821741683684368e-2_f64 * t44609 * t3720 * t44610 * t1250 + 0.34299214494455789578e-2_f64 * t44616 + 0.25724410870841842184e-2_f64 * t12910 * t3720 * t44618 * t1250 + 0.51448821741683684368e-2_f64 * t44624 * t12912 - 0.28582678745379824648e-2_f64 * t17729 * t12787 * t3362 * t2258 * t17730 + 0.22866142996303859718e-2_f64 * t44634 + 0.17149607247227894789e-2_f64 * t44637 + 0.17149607247227894789e-2_f64 * t5340 * t3720 * t44639 * t5341 - 0.85748036236139473944e-3_f64 * t5331 * t3720 * t44639 * t5333 + 0.11433071498151929859e-2_f64 * t44649 - 0.34299214494455789578e-2_f64 * t17261 * t13081 + 0.2540682555144873302e-2_f64 * t3711 * t1042 * t17235 * t44205;
    (t44639, t44657)
}
