//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 917/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk917<F: Float>(t25464: F, t27695: F, t1647: F, t1976: F, t7817: F, t999: F, t7145: F, t1097: F, t1983: F, t25473: F, t25591: F, t25605: F, t25611: F, t25629: F, t25699: F, t27653: F, t27656: F, t27661: F, t27665: F, t27669: F, t27670: F, t27676: F, t27680: F, t27684: F, t27688: F, t27692: F, t7144: F, t7147: F, t7151: F, t7159: F, t7812: F, t7829: F, t989: F) -> F {
    let t27696 = t25464 * t27695;
    let t27699 = t1647 * t1976;
    let t27702 = t7817 * t999;
    let t27703 = t7145 * t27702;
    let t27706 = -F::cast_from(0.8673628188205199462e0_f64) * t25629 * t27653 + F::cast_from(0.8673628188205199462e0_f64) * t25611 * t27656 + F::cast_from(0.8673628188205199462e0_f64) * t25473 * t7829 - F::cast_from(0.8673628188205199462e0_f64) * t27661 * t7147 + F::cast_from(0.8673628188205199462e0_f64) * t25605 * t27665 - F::cast_from(0.8673628188205199462e0_f64) * t27669 * t27670 + F::cast_from(0.65854491829355115987e0_f64) * t989 * t7812 - F::cast_from(0.4336814094102599731e0_f64) * t1983 * t27676 - F::cast_from(0.8673628188205199462e0_f64) * t7144 * t27680 - F::cast_from(0.17347256376410398924e1_f64) * t7151 * t27684 + F::cast_from(0.8673628188205199462e0_f64) * t7151 * t27688 - F::cast_from(0.26020884564615598386e1_f64) * t25699 * t27692 - F::cast_from(0.26020884564615598386e1_f64) * t7159 * t27696 - F::cast_from(0.65854491829355115987e0_f64) * t27699 * t1097 + F::cast_from(0.17347256376410398924e1_f64) * t25591 * t27703;
    t27706
}
