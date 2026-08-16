//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2068/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2068<F: Float>(t25471: F, t82431: F, t7607: F, t82632: F, t25490: F, t82514: F, t3030: F, t343: F, t25483: F, t25486: F, t25492: F, t23478: F, t4547: F) -> (F, F, F, F, F, F) {
    let t89445 = F::cast_from(0.18277045187202515961e-2_f64) * t82431 * t25471;
    let t89449 = t82632 * t7607;
    let t89468 = t82514 * t25490;
    let t89499 = t343 * t3030;
    let t89501 = t89499 * t25483 * t25486;
    let t89505 = t89499 * t25490 * t25492;
    let t89532 = t4547 * t23478;
    (t89445, t89449, t89468, t89501, t89505, t89532)
}
