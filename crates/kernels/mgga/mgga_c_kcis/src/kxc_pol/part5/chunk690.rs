//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 690/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk690<F: Float>(t1897: F, t544: F, t1319: F, t5457: F, t518: F, t1419: F, t3786: F, t1890: F, t653: F) -> (F, F, F, F, F, F, F) {
    let t5458 = t544 * t1897;
    let t5459 = t5458 * t1319;
    let t5460 = t5457 * t5459;
    let t5463 = t518 * t1897;
    let t5464 = t5463 * t1419;
    let t5465 = t3786 * t5464;
    let t5469 = t653 * t1890;
    (t5458, t5459, t5460, t5463, t5464, t5465, t5469)
}
