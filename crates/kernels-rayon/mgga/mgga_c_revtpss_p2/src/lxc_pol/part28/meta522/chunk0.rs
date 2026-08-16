//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1944/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1944(t25464: f64, t27695: f64, t1647: f64, t1976: f64, t7817: f64, t999: f64, t7145: f64, t1097: f64, t1983: f64, t25473: f64, t25591: f64, t25605: f64, t25611: f64, t25629: f64, t25699: f64, t27653: f64, t27656: f64, t27661: f64, t27665: f64, t27669: f64, t27670: f64, t27676: f64, t27680: f64, t27684: f64, t27688: f64, t27692: f64, t7144: f64, t7147: f64, t7151: f64, t7159: f64, t7812: f64, t7829: f64, t989: f64) -> (f64, f64, f64, f64, f64) {
    let t27696 = t25464 * t27695;
    let t27699 = t1647 * t1976;
    let t27702 = t7817 * t999;
    let t27703 = t7145 * t27702;
    let t27706 = -0.8673628188205199462e0_f64 * t25629 * t27653 + 0.8673628188205199462e0_f64 * t25611 * t27656 + 0.8673628188205199462e0_f64 * t25473 * t7829 - 0.8673628188205199462e0_f64 * t27661 * t7147 + 0.8673628188205199462e0_f64 * t25605 * t27665 - 0.8673628188205199462e0_f64 * t27669 * t27670 + 0.65854491829355115987e0_f64 * t989 * t7812 - 0.4336814094102599731e0_f64 * t1983 * t27676 - 0.8673628188205199462e0_f64 * t7144 * t27680 - 0.17347256376410398924e1_f64 * t7151 * t27684 + 0.8673628188205199462e0_f64 * t7151 * t27688 - 0.26020884564615598386e1_f64 * t25699 * t27692 - 0.26020884564615598386e1_f64 * t7159 * t27696 - 0.65854491829355115987e0_f64 * t27699 * t1097 + 0.17347256376410398924e1_f64 * t25591 * t27703;
    (t27696, t27699, t27702, t27703, t27706)
}
