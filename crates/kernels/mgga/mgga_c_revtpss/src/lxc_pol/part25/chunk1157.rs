//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1157/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1157<F: Float>(t1097: F, t1983: F, t1986: F, t25461: F, t25466: F, t25470: F, t25473: F, t25476: F, t25480: F, t25484: F, t25487: F, t25588: F, t25591: F, t25593: F, t25597: F, t25601: F, t25605: F, t25607: F, t25611: F, t25613: F, t25617: F, t25621: F, t25626: F, t25629: F, t25631: F, t25634: F, t7144: F, t7147: F, t7151: F, t7153: F, t7159: F, t7162: F) -> F {
    let t25637 = F::new(0.17347256376410398924e1) * t25461 * t7153 - F::new(0.26020884564615598386e1) * t7159 * t25466 + F::new(0.17347256376410398924e1) * t7159 * t25470 + F::new(0.17347256376410398924e1) * t25473 * t7162 - F::new(0.17347256376410398924e1) * t25476 * t7147 + F::new(0.8673628188205199462e0) * t7159 * t25480 + F::new(0.8673628188205199462e0) * t7151 * t25484 - F::new(0.4336814094102599731e0) * t25487 * t1986 - F::new(0.4336814094102599731e0) * t1983 * t25588 + F::new(0.34694512752820797848e1) * t25591 * t25593 - F::new(0.34694512752820797848e1) * t7151 * t25597 + F::new(0.34694512752820797848e1) * t7144 * t25601 + F::new(0.17347256376410398924e1) * t25605 * t25607 + F::new(0.17347256376410398924e1) * t25611 * t25613 + F::new(0.17347256376410398924e1) * t7151 * t25617 - F::new(0.8673628188205199462e0) * t7144 * t25621 - F::new(0.8673628188205199462e0) * t25626 * t1986 - F::new(0.17347256376410398924e1) * t25629 * t25631 - F::new(0.13170898365871023197e1) * t25634 * t1097;
    t25637
}
