//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 146/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk146<F: Float>(t397: F, t399: F, t539: F, t535: F, t473: F, t524: F, t489: F, t501: F, t240: F, t505: F, rho1: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t541 = t397 * t399 * t539;
    let t544 = 1.0 + 0.2698618307426597582e-1 * t535 * t541;
    let t545 = f64::ln(t544);
    let t547 = 1.0 + 0.193e0 * t545;
    let t548 = 1.0 / t547;
    let t551 = t524 * t548 + 0.17411041666666666666e-2 * t473;
    let t554 = 1.0 + 0.9375e-1 * t489 - 0.101171875e-1 * t501;
    let t555 = 1.0 / t554;
    let t559 = t505 + t240 * (t551 * t555 - t505);
    let t563 = 1.0 / rho1;
    let t564 = sigma2 * t563;
    (t541, t544, t547, t548, t551, t554, t555, t559, t564)
}
