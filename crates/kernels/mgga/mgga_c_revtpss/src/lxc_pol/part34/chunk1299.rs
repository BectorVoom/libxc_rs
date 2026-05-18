//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1299/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1299<F: Float>(t107629: F, t1089: F, t1646: F, t1668: F, t1678: F, t1695: F, t1696: F, t1976: F, t1986: F, t23599: F, t23603: F, t24047: F, t24177: F, t25464: F, t27550: F, t27616: F, t27661: F, t29727: F, t29748: F, t29751: F, t29834: F, t29871: F, t6234: F, t6251: F, t6258: F, t6351: F, t6392: F, t7102: F, t7140: F, t7144: F, t7145: F, t7151: F, t7159: F, t7160: F, t7810: F, t93968: F, t94122: F) -> F {
    let t113961 = -F::new(0.65854491829355115987e0) * t7102 * t23599 + F::new(0.26020884564615598386e1) * t7151 * t7145 * t7810 * t6258 - F::new(0.15612530738769359031e2) * t7144 * t25464 * t29751 * t1646 + F::new(0.26020884564615598386e1) * t7159 * t7160 * t7810 * t6392 + F::new(0.10408353825846239354e2) * t7159 * t93968 * t1976 * t24047 - F::new(0.26020884564615598386e1) * t7144 * t7145 * t7810 * t6234 + F::new(0.39512695097613069591e1) * t7140 * t23603 + F::new(0.8673628188205199462e0) * t7159 * t7160 * t1976 * t24177 - F::new(0.78062653693846795158e1) * t94122 * t29871 * t1668 * t1089 - F::new(0.52041769129231196772e1) * t27661 * t29748 - F::new(0.26020884564615598386e1) * t29834 * t1678 * t1986 + F::new(0.39512695097613069591e1) * t27616 * t6351 - F::new(0.39512695097613069591e1) * t107629 * t1696 + F::new(0.39512695097613069591e1) * t27550 * t6251 - F::new(0.10408353825846239354e2) * t7151 * t7160 * t29727 * t1695;
    t113961
}
