//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2262/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2262<F: Float>(t1032: F, t6695: F, t2148: F, t1209: F, t105442: F, t111987: F, t1248: F, t1287: F, t20760: F, t21618: F, t21624: F, t26889: F, t26918: F, t26994: F, t27008: F, t29220: F, t29224: F, t29275: F, t29304: F, t30743: F, t30763: F, t30874: F, t5245: F, t5423: F, t6745: F, t7602: F, t7632: F, t7637: F, t7639: F, t7643: F, t7654: F, t8190: F, t8197: F, t8198: F, t97313: F) -> (F, F) {
    let t112757 = t6695 * t1032;
    let t112758 = t2148 * t112757;
    let t112774 = t1209 * t112757;
    let t112787 = F::cast_from(0.13170898365871023197e1_f64) * t29304 * t5423 - F::cast_from(0.17347256376410398924e1_f64) * t26889 * t30743 * t1248 * t1287 - F::cast_from(0.65854491829355115987e0_f64) * t27008 * t6745 - F::cast_from(0.65854491829355115987e0_f64) * t7602 * t21624 + F::cast_from(0.8673628188205199462e0_f64) * t112758 * t7654 + F::cast_from(0.17347256376410398924e1_f64) * t7643 * t7637 * t8190 * t5245 + F::cast_from(0.13170898365871023197e1_f64) * t29220 * t5423 - F::cast_from(0.17347256376410398924e1_f64) * t105442 * t8198 - F::cast_from(0.65854491829355115987e0_f64) * t7632 * t21618 + F::cast_from(0.34694512752820797848e1_f64) * t97313 * t30763 * t111987 - F::cast_from(0.8673628188205199462e0_f64) * t112774 * t7639 + F::cast_from(0.13170898365871023197e1_f64) * t7632 * t20760 + F::cast_from(0.34694512752820797848e1_f64) * t26994 * t7637 * t8197 * t5245 + F::cast_from(0.34694512752820797848e1_f64) * t29275 * t29224 - F::cast_from(0.4336814094102599731e0_f64) * t26918 * t30874;
    (t112757, t112787)
}
