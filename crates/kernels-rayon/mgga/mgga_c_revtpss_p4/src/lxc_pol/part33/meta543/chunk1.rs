//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1916/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1916(t1287: f64, t5284: f64, t7660: f64, t1215: f64, t1295: f64, t1770: f64, t2144: f64, t26889: f64, t26895: f64, t26918: f64, t26976: f64, t26979: f64, t29213: f64, t29217: f64, t29220: f64, t29224: f64, t29227: f64, t29233: f64, t29237: f64, t29247: f64, t5216: f64, t5231: f64, t5423: f64, t7602: f64, t7629: f64, t7636: f64, t7643: f64, t7659: f64, t8202: f64, t8213: f64) -> (f64, f64) {
    let t29251 = t7660 * t5284 * t1287;
    let t29258 = 0.8673628188205199462e0_f64 * t26895 * t29213 - 0.8673628188205199462e0_f64 * t26889 * t29217 - 0.65854491829355115987e0_f64 * t29220 * t1215 + 0.17347256376410398924e1_f64 * t7636 * t29224 - 0.65854491829355115987e0_f64 * t29227 * t1295 + 0.8673628188205199462e0_f64 * t26979 * t8202 + 0.8673628188205199462e0_f64 * t7643 * t29233 - 0.17347256376410398924e1_f64 * t7643 * t29237 + 0.65854491829355115987e0_f64 * t5216 * t2144 + 0.65854491829355115987e0_f64 * t1770 * t7629 - 0.4336814094102599731e0_f64 * t26918 * t8213 - 0.4336814094102599731e0_f64 * t7659 * t29247 - 0.4336814094102599731e0_f64 * t7659 * t29251 + 0.13170898365871023197e1_f64 * t26976 * t5231 + 0.65854491829355115987e0_f64 * t7602 * t5423;
    (t29251, t29258)
}
