//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 773/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk773<F: Float>(t1646: F, t7135: F, t7145: F, t7828: F, t999: F, t7160: F, t1651: F, t7821: F, t1096: F, t25464: F, t1647: F, t1976: F, t7817: F, t1097: F, t1983: F, t25473: F, t25591: F, t25605: F, t25611: F, t25629: F, t25699: F, t27653: F, t27656: F, t27661: F, t27665: F, t27669: F, t27670: F, t27676: F, t7144: F, t7147: F, t7151: F, t7159: F, t7812: F, t7829: F, t989: F) -> (F, F, F) {
    let t27679 = t7135 * t1646;
    let t27680 = t7145 * t27679;
    let t27683 = t7828 * t999;
    let t27684 = t7160 * t27683;
    let t27687 = t7135 * t1651;
    let t27688 = t7145 * t27687;
    let t27691 = t7821 * t999;
    let t27692 = t7145 * t27691;
    let t27695 = t7828 * t1096;
    let t27696 = t25464 * t27695;
    let t27699 = t1647 * t1976;
    let t27702 = t7817 * t999;
    let t27703 = t7145 * t27702;
    let t27706 = -0.8673628188205199462e0 * t25629 * t27653 + 0.8673628188205199462e0 * t25611 * t27656 + 0.8673628188205199462e0 * t25473 * t7829 - 0.8673628188205199462e0 * t27661 * t7147 + 0.8673628188205199462e0 * t25605 * t27665 - 0.8673628188205199462e0 * t27669 * t27670 + 0.65854491829355115987e0 * t989 * t7812 - 0.4336814094102599731e0 * t1983 * t27676 - 0.8673628188205199462e0 * t7144 * t27680 - 0.17347256376410398924e1 * t7151 * t27684 + 0.8673628188205199462e0 * t7151 * t27688 - 0.26020884564615598386e1 * t25699 * t27692 - 0.26020884564615598386e1 * t7159 * t27696 - 0.65854491829355115987e0 * t27699 * t1097 + 0.17347256376410398924e1 * t25591 * t27703;
    (t27680, t27688, t27706)
}
