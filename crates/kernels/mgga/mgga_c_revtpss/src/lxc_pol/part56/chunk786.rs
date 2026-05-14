//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 786/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk786<F: Float>(t29186: F, t7637: F, t11239: F, t1276: F, t3596: F, t2149: F, t29157: F, t3153: F, t5465: F, t1243: F, t5480: F, t3555: F, t7635: F, t460: F, t8190: F, t1204: F, t1295: F, t1775: F, t1829: F, t26889: F, t26895: F, t26922: F, t26937: F, t26999: F, t27020: F, t29160: F, t29163: F, t29167: F, t29175: F, t29179: F, t29183: F, t7636: F, t7651: F, t8192: F, t8198: F, t8209: F) -> (F, F) {
    let t29187 = t7637 * t29186;
    let t29192 = t11239 * t1276;
    let t29193 = t29192 * t3596;
    let t29194 = t2149 * t29193;
    let t29195 = t29157 * t3153;
    let t29196 = t29195 * t5465;
    let t29199 = t29192 * t1243;
    let t29200 = t2149 * t29199;
    let t29201 = t29195 * t5480;
    let t29204 = t3555 * t7635;
    let t29207 = t460 * t8190;
    let t29210 = 0.65854491829355115987e0 * t1204 * t8192 - 0.8673628188205199462e0 * t26889 * t29160 + 0.8673628188205199462e0 * t26895 * t29163 + 0.8673628188205199462e0 * t26922 * t29167 - 0.65854491829355115987e0 * t26999 * t1775 + 0.8673628188205199462e0 * t26937 * t8209 - 0.8673628188205199462e0 * t7636 * t29175 + 0.8673628188205199462e0 * t7651 * t29179 + 0.65854491829355115987e0 * t460 * t29183 - 0.8673628188205199462e0 * t7636 * t29187 - 0.65854491829355115987e0 * t27020 * t1829 - 0.8673628188205199462e0 * t29194 * t29196 + 0.4336814094102599731e0 * t29200 * t29201 - 0.8673628188205199462e0 * t29204 * t8198 - 0.65854491829355115987e0 * t29207 * t1295;
    (t29187, t29210)
}
