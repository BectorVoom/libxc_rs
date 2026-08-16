//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 996/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk996(t29195: f64, t5465: f64, t1243: f64, t29192: f64, t2149: f64, t5480: f64, t3555: f64, t7635: f64, t460: f64, t8190: f64, t1204: f64, t1295: f64, t1775: f64, t1829: f64, t26889: f64, t26895: f64, t26922: f64, t26937: f64, t26999: f64, t27020: f64, t29160: f64, t29163: f64, t29167: f64, t29175: f64, t29179: f64, t29183: f64, t29187: f64, t29194: f64, t7636: f64, t7651: f64, t8192: f64, t8198: f64, t8209: f64) -> f64 {
    let t29196 = t29195 * t5465;
    let t29199 = t29192 * t1243;
    let t29200 = t2149 * t29199;
    let t29201 = t29195 * t5480;
    let t29204 = t3555 * t7635;
    let t29207 = t460 * t8190;
    let t29210 = 0.65854491829355115987e0_f64 * t1204 * t8192 - 0.8673628188205199462e0_f64 * t26889 * t29160 + 0.8673628188205199462e0_f64 * t26895 * t29163 + 0.8673628188205199462e0_f64 * t26922 * t29167 - 0.65854491829355115987e0_f64 * t26999 * t1775 + 0.8673628188205199462e0_f64 * t26937 * t8209 - 0.8673628188205199462e0_f64 * t7636 * t29175 + 0.8673628188205199462e0_f64 * t7651 * t29179 + 0.65854491829355115987e0_f64 * t460 * t29183 - 0.8673628188205199462e0_f64 * t7636 * t29187 - 0.65854491829355115987e0_f64 * t27020 * t1829 - 0.8673628188205199462e0_f64 * t29194 * t29196 + 0.4336814094102599731e0_f64 * t29200 * t29201 - 0.8673628188205199462e0_f64 * t29204 * t8198 - 0.65854491829355115987e0_f64 * t29207 * t1295;
    t29210
}
