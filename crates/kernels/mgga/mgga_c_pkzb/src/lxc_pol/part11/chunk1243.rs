//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1243/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1243<F: Float>(t30525: F, t30541: F, t10830: F, t10834: F, t17633: F, t1950: F, t20908: F, t30263: F, t30265: F, t30268: F, t30362: F, t30364: F, t30366: F, t30369: F, t30381: F, t30385: F, t30387: F, t30502: F, t714: F, t722: F, t9423: F) -> (F, F) {
    let t30542 = t30525 + t30541;
    let t30548 = t30263 + t30265 + t30268 - t30362 - t30364 - t30366 - t30369 - t30381 - t30385 - t30387 - t30502 - F::new(0.57895126195293126241e3) * t20908 * t9423 + F::new(0.5848223622634646207e0) * t1950 * t10830 + F::new(0.5848223622634646207e0) * t714 * t30542 * t722 + F::new(0.10254018858216406658e4) * t17633 * t10834;
    (t30542, t30548)
}
