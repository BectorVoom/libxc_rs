//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 803/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk803<F: Float>(t11690: F, t11693: F, t11696: F, t11698: F, t11704: F, t11707: F, t11936: F, t12399: F, t240: F, t567: F, t564: F, t1152: F, t3477: F) -> (F, F) {
    let t12401 = t12399 * t240 + t11690 - t11693 + t11696 - t11698 - t11704 + t11707 - t11936;
    let t12402 = t567 * t12401;
    let t12403 = t564 * t12402;
    let t12404 = t12403 / F::new(16.0);
    let t12405 = t1152 * t3477;
    (t12404, t12405)
}
