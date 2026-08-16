//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2262/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2262(t1032: f64, t6695: f64, t2148: f64, t1209: f64, t105442: f64, t111987: f64, t1248: f64, t1287: f64, t20760: f64, t21618: f64, t21624: f64, t26889: f64, t26918: f64, t26994: f64, t27008: f64, t29220: f64, t29224: f64, t29275: f64, t29304: f64, t30743: f64, t30763: f64, t30874: f64, t5245: f64, t5423: f64, t6745: f64, t7602: f64, t7632: f64, t7637: f64, t7639: f64, t7643: f64, t7654: f64, t8190: f64, t8197: f64, t8198: f64, t97313: f64) -> (f64, f64) {
    let t112757 = t6695 * t1032;
    let t112758 = t2148 * t112757;
    let t112774 = t1209 * t112757;
    let t112787 = 0.13170898365871023197e1_f64 * t29304 * t5423 - 0.17347256376410398924e1_f64 * t26889 * t30743 * t1248 * t1287 - 0.65854491829355115987e0_f64 * t27008 * t6745 - 0.65854491829355115987e0_f64 * t7602 * t21624 + 0.8673628188205199462e0_f64 * t112758 * t7654 + 0.17347256376410398924e1_f64 * t7643 * t7637 * t8190 * t5245 + 0.13170898365871023197e1_f64 * t29220 * t5423 - 0.17347256376410398924e1_f64 * t105442 * t8198 - 0.65854491829355115987e0_f64 * t7632 * t21618 + 0.34694512752820797848e1_f64 * t97313 * t30763 * t111987 - 0.8673628188205199462e0_f64 * t112774 * t7639 + 0.13170898365871023197e1_f64 * t7632 * t20760 + 0.34694512752820797848e1_f64 * t26994 * t7637 * t8197 * t5245 + 0.34694512752820797848e1_f64 * t29275 * t29224 - 0.4336814094102599731e0_f64 * t26918 * t30874;
    (t112757, t112787)
}
