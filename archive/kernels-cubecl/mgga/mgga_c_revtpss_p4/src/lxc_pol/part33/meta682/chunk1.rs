//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2234/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2234<F: Float>(t1209: F, t30840: F, t111844: F, t3153: F, t104510: F, t105519: F, t1215: F, t1287: F, t1294: F, t1794: F, t1829: F, t20722: F, t21366: F, t26889: F, t26922: F, t26949: F, t26976: F, t27020: F, t29135: F, t29174: F, t29178: F, t29194: F, t29200: F, t29204: F, t30739: F, t30743: F, t30744: F, t30763: F, t30850: F, t3555: F, t5284: F, t5465: F, t5480: F, t6703: F, t7602: F, t7636: F, t7652: F, t8197: F, t8198: F, t96927: F, t97082: F) -> F {
    let t111865 = t1209 * t30840;
    let t111906 = t111844 * t3153;
    let t111913 = -F::cast_from(0.65854491829355115987e0_f64) * t111865 * t1215 + F::cast_from(0.34694512752820797848e1_f64) * t7636 * t7652 * t30743 * t1294 + F::cast_from(0.13170898365871023197e1_f64) * t27020 * t6703 + F::cast_from(0.13170898365871023197e1_f64) * t7602 * t21366 + F::cast_from(0.52041769129231196772e1_f64) * t26949 * t7652 * t30739 * t1294 + F::cast_from(0.26341796731742046394e1_f64) * t26976 * t20722 - F::cast_from(0.17347256376410398924e1_f64) * t29204 * t30744 - F::cast_from(0.34694512752820797848e1_f64) * t96927 * t30763 * t104510 - F::cast_from(0.13170898365871023197e1_f64) * t105519 * t1829 - F::cast_from(0.17347256376410398924e1_f64) * t26889 * t29174 * t1794 * t1287 - F::cast_from(0.17347256376410398924e1_f64) * t26889 * t8197 * t5284 * t1287 + F::cast_from(0.17347256376410398924e1_f64) * t97082 * t30850 + F::cast_from(0.17347256376410398924e1_f64) * t26922 * t29178 * t1794 * t1287 - F::cast_from(0.17347256376410398924e1_f64) * t3555 * t29135 * t8198 - F::cast_from(0.8673628188205199462e0_f64) * t29194 * t111906 * t5465 + F::cast_from(0.4336814094102599731e0_f64) * t29200 * t111906 * t5480;
    t111913
}
