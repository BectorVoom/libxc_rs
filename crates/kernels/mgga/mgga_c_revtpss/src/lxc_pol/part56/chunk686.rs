//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 686/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk686<F: Float>(t1276: F, t7657: F, t2148: F, t1243: F, t2142: F, t1248: F, t1287: F, t2150: F, t473: F, t7627: F, t1204: F, t1215: F, t1295: F, t2144: F, t2149: F, t2152: F, t460: F, t7602: F, t7629: F, t7632: F, t7636: F, t7639: F, t7643: F, t7645: F, t7648: F, t7651: F, t7654: F) -> (F, F, F, F, F) {
    let t7658 = t7657 * t1276;
    let t7659 = t2148 * t7658;
    let t7660 = t1243 * t2142;
    let t7662 = t7660 * t1248 * t1287;
    let t7666 = t2150 * t473 * t7627;
    let t7669 = F::new(0.65854491829355115987e0) * t1204 * t2144 - F::new(0.65854491829355115987e0) * t7602 * t1215 + F::new(0.65854491829355115987e0) * t460 * t7629 - F::new(0.65854491829355115987e0) * t7632 * t1295 - F::new(0.8673628188205199462e0) * t7636 * t7639 + F::new(0.8673628188205199462e0) * t7643 * t7645 - F::new(0.4336814094102599731e0) * t7648 * t2152 + F::new(0.8673628188205199462e0) * t7651 * t7654 - F::new(0.4336814094102599731e0) * t7659 * t7662 - F::new(0.4336814094102599731e0) * t2149 * t7666;
    (t7659, t7660, t7662, t7666, t7669)
}
