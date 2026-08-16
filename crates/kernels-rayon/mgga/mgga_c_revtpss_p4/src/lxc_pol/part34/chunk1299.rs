//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1299/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1299(t107629: f64, t1089: f64, t1646: f64, t1668: f64, t1678: f64, t1695: f64, t1696: f64, t1976: f64, t1986: f64, t23599: f64, t23603: f64, t24047: f64, t24177: f64, t25464: f64, t27550: f64, t27616: f64, t27661: f64, t29727: f64, t29748: f64, t29751: f64, t29834: f64, t29871: f64, t6234: f64, t6251: f64, t6258: f64, t6351: f64, t6392: f64, t7102: f64, t7140: f64, t7144: f64, t7145: f64, t7151: f64, t7159: f64, t7160: f64, t7810: f64, t93968: f64, t94122: f64) -> f64 {
    let t113961 = -0.65854491829355115987e0_f64 * t7102 * t23599 + 0.26020884564615598386e1_f64 * t7151 * t7145 * t7810 * t6258 - 0.15612530738769359031e2_f64 * t7144 * t25464 * t29751 * t1646 + 0.26020884564615598386e1_f64 * t7159 * t7160 * t7810 * t6392 + 0.10408353825846239354e2_f64 * t7159 * t93968 * t1976 * t24047 - 0.26020884564615598386e1_f64 * t7144 * t7145 * t7810 * t6234 + 0.39512695097613069591e1_f64 * t7140 * t23603 + 0.8673628188205199462e0_f64 * t7159 * t7160 * t1976 * t24177 - 0.78062653693846795158e1_f64 * t94122 * t29871 * t1668 * t1089 - 0.52041769129231196772e1_f64 * t27661 * t29748 - 0.26020884564615598386e1_f64 * t29834 * t1678 * t1986 + 0.39512695097613069591e1_f64 * t27616 * t6351 - 0.39512695097613069591e1_f64 * t107629 * t1696 + 0.39512695097613069591e1_f64 * t27550 * t6251 - 0.10408353825846239354e2_f64 * t7151 * t7160 * t29727 * t1695;
    t113961
}
