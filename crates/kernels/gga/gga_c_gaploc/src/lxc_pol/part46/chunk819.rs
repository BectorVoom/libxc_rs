//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 819/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk819<F: Float>(t41609: F, t10469: F, t2482: F, t9267: F, t2476: F, t26922: F, t9438: F, t10268: F, t4391: F, t549: F, t2365: F, t31748: F) -> (F, F, F, F, F) {
    let t41610 = F::new(0.15337170381568299871e1) * t41609;
    let t41612 = t9267 * t10469 * t2482;
    let t41613 = F::new(0.72851559312449424385e1) * t41612;
    let t41615 = t2476 * t9438 * t26922;
    let t41616 = F::new(0.15976219147466979032e-1) * t41615;
    let t41618 = t4391 * t549 * t10268;
    let t41619 = F::new(0.11916829983950142223e0) * t41618;
    let t41621 = t4391 * t2365 * t31748;
    (t41610, t41613, t41616, t41619, t41621)
}
