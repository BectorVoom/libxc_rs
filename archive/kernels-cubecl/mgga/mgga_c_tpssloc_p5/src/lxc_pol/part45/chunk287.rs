//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 287/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk287<F: Float>(t248: F, t557: F, t836: F, t555: F, t236: F, t552: F, t240: F, t1336: F, t531: F, t556: F, t241: F, t67: F) -> (F, F, F, F, F, F, F) {
    let t1358 = t836 * t557 * t248;
    let t1360 = F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t555 * t1358;
    let t1361 = t552 * t236;
    let t1362 = t1361 * t240;
    let t1363 = t1336 * t1362;
    let t1365 = F::cast_from(1.0_f64) / t556 / t531;
    let t1367 = t241 * t1365 * t67;
    (t1358, t1360, t1361, t1362, t1363, t1365, t1367)
}
