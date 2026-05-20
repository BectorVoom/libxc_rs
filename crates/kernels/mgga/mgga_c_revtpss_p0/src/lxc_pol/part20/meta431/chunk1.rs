//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1625/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1625<F: Float>(t12732: F, t3153: F, t12952: F, t3172: F, t3711: F, t1042: F, t1250: F, t12787: F, t12910: F, t12912: F, t13081: F, t17235: F, t17261: F, t17729: F, t17730: F, t2258: F, t3362: F, t3718: F, t3720: F, t44205: F, t44599: F, t44607: F, t44609: F, t44610: F, t44616: F, t44618: F, t44624: F, t44634: F, t44637: F, t5331: F, t5333: F, t5340: F, t5341: F) -> (F, F) {
    let t44639 = t12732 * t3153;
    let t44649 = t3711 * t3172 * t12952;
    let t44657 = -F::cast_from(0.85748036236139473944e-3_f64) * t3718 * t3720 * t44599 * t1250 - t44607 - F::cast_from(0.51448821741683684368e-2_f64) * t44609 * t3720 * t44610 * t1250 + F::cast_from(0.34299214494455789578e-2_f64) * t44616 + F::cast_from(0.25724410870841842184e-2_f64) * t12910 * t3720 * t44618 * t1250 + F::cast_from(0.51448821741683684368e-2_f64) * t44624 * t12912 - F::cast_from(0.28582678745379824648e-2_f64) * t17729 * t12787 * t3362 * t2258 * t17730 + F::cast_from(0.22866142996303859718e-2_f64) * t44634 + F::cast_from(0.17149607247227894789e-2_f64) * t44637 + F::cast_from(0.17149607247227894789e-2_f64) * t5340 * t3720 * t44639 * t5341 - F::cast_from(0.85748036236139473944e-3_f64) * t5331 * t3720 * t44639 * t5333 + F::cast_from(0.11433071498151929859e-2_f64) * t44649 - F::cast_from(0.34299214494455789578e-2_f64) * t17261 * t13081 + F::cast_from(0.2540682555144873302e-2_f64) * t3711 * t1042 * t17235 * t44205;
    (t44639, t44657)
}
