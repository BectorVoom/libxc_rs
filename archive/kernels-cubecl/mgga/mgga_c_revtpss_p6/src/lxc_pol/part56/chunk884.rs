//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 884/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk884<F: Float>(t1294: F, t8201: F, t7652: F, t1287: F, t1794: F, t26931: F, t5284: F, t7660: F, t1215: F, t1295: F, t1770: F, t2144: F, t26889: F, t26895: F, t26918: F, t26976: F, t26979: F, t29213: F, t29217: F, t29220: F, t29224: F, t29227: F, t29233: F, t5216: F, t5231: F, t5423: F, t7602: F, t7629: F, t7636: F, t7643: F, t7659: F, t8202: F, t8213: F) -> (F, F) {
    let t29236 = t8201 * t1294;
    let t29237 = t7652 * t29236;
    let t29247 = t26931 * t1794 * t1287;
    let t29251 = t7660 * t5284 * t1287;
    let t29258 = F::cast_from(0.8673628188205199462e0_f64) * t26895 * t29213 - F::cast_from(0.8673628188205199462e0_f64) * t26889 * t29217 - F::cast_from(0.65854491829355115987e0_f64) * t29220 * t1215 + F::cast_from(0.17347256376410398924e1_f64) * t7636 * t29224 - F::cast_from(0.65854491829355115987e0_f64) * t29227 * t1295 + F::cast_from(0.8673628188205199462e0_f64) * t26979 * t8202 + F::cast_from(0.8673628188205199462e0_f64) * t7643 * t29233 - F::cast_from(0.17347256376410398924e1_f64) * t7643 * t29237 + F::cast_from(0.65854491829355115987e0_f64) * t5216 * t2144 + F::cast_from(0.65854491829355115987e0_f64) * t1770 * t7629 - F::cast_from(0.4336814094102599731e0_f64) * t26918 * t8213 - F::cast_from(0.4336814094102599731e0_f64) * t7659 * t29247 - F::cast_from(0.4336814094102599731e0_f64) * t7659 * t29251 + F::cast_from(0.13170898365871023197e1_f64) * t26976 * t5231 + F::cast_from(0.65854491829355115987e0_f64) * t7602 * t5423;
    (t29247, t29258)
}
