//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1913/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1913<F: Float>(t29195: F, t5465: F, t1243: F, t29192: F, t2149: F, t5480: F, t3555: F, t7635: F, t460: F, t8190: F, t1204: F, t1295: F, t1775: F, t1829: F, t26889: F, t26895: F, t26922: F, t26937: F, t26999: F, t27020: F, t29160: F, t29163: F, t29167: F, t29175: F, t29179: F, t29183: F, t29187: F, t29194: F, t7636: F, t7651: F, t8192: F, t8198: F, t8209: F) -> (F, F, F, F, F, F, F) {
    let t29196 = t29195 * t5465;
    let t29199 = t29192 * t1243;
    let t29200 = t2149 * t29199;
    let t29201 = t29195 * t5480;
    let t29204 = t3555 * t7635;
    let t29207 = t460 * t8190;
    let t29210 = F::cast_from(0.65854491829355115987e0_f64) * t1204 * t8192 - F::cast_from(0.8673628188205199462e0_f64) * t26889 * t29160 + F::cast_from(0.8673628188205199462e0_f64) * t26895 * t29163 + F::cast_from(0.8673628188205199462e0_f64) * t26922 * t29167 - F::cast_from(0.65854491829355115987e0_f64) * t26999 * t1775 + F::cast_from(0.8673628188205199462e0_f64) * t26937 * t8209 - F::cast_from(0.8673628188205199462e0_f64) * t7636 * t29175 + F::cast_from(0.8673628188205199462e0_f64) * t7651 * t29179 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t29183 - F::cast_from(0.8673628188205199462e0_f64) * t7636 * t29187 - F::cast_from(0.65854491829355115987e0_f64) * t27020 * t1829 - F::cast_from(0.8673628188205199462e0_f64) * t29194 * t29196 + F::cast_from(0.4336814094102599731e0_f64) * t29200 * t29201 - F::cast_from(0.8673628188205199462e0_f64) * t29204 * t8198 - F::cast_from(0.65854491829355115987e0_f64) * t29207 * t1295;
    (t29196, t29199, t29200, t29201, t29204, t29207, t29210)
}
