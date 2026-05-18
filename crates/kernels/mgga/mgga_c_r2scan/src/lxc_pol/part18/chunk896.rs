//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 896/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk896<F: Float>(t1048: F, t795: F, t9573: F, t910: F, t2266: F, t2867: F, t2267: F, t2892: F, t2858: F, t2526: F, t2859: F, t2333: F, t3245: F) -> (F, F, F, F, F) {
    let t9575 = t1048 * t9573 * t795;
    let t9576 = F::new(2.0) * t9575;
    let t9577 = t910 * t795;
    let t9579 = t2266 * t2867 * t9577;
    let t9580 = F::new(6.0) * t9579;
    let t9583 = t2267 * t2892;
    let t9584 = t2858 * t9583;
    let t9585 = F::new(6.0) * t9584;
    let t9586 = t2859 * t2526;
    let t9587 = t2858 * t9586;
    let t9588 = F::new(12.0) * t9587;
    let t9589 = t3245 * t2333;
    (t9576, t9580, t9585, t9588, t9589)
}
