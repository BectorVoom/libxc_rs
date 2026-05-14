//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 865/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk865<F: Float>(t1215: F, t1295: F, t1770: F, t2144: F, t26889: F, t26895: F, t26918: F, t26976: F, t26979: F, t29213: F, t29217: F, t29220: F, t29224: F, t29227: F, t29233: F, t29237: F, t29247: F, t29251: F, t5216: F, t5231: F, t5423: F, t7602: F, t7629: F, t7636: F, t7643: F, t7659: F, t8202: F, t8213: F) -> (F,) {
    let t29258 = 0.8673628188205199462e0 * t26895 * t29213 - 0.8673628188205199462e0 * t26889 * t29217 - 0.65854491829355115987e0 * t29220 * t1215 + 0.17347256376410398924e1 * t7636 * t29224 - 0.65854491829355115987e0 * t29227 * t1295 + 0.8673628188205199462e0 * t26979 * t8202 + 0.8673628188205199462e0 * t7643 * t29233 - 0.17347256376410398924e1 * t7643 * t29237 + 0.65854491829355115987e0 * t5216 * t2144 + 0.65854491829355115987e0 * t1770 * t7629 - 0.4336814094102599731e0 * t26918 * t8213 - 0.4336814094102599731e0 * t7659 * t29247 - 0.4336814094102599731e0 * t7659 * t29251 + 0.13170898365871023197e1 * t26976 * t5231 + 0.65854491829355115987e0 * t7602 * t5423;
    (t29258,)
}
