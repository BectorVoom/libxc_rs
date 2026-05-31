//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1427/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1427<F: Float>(t1616: F, t2011: F, t3909: F, t1617: F, t4915: F, t1615: F, t3903: F, t12585: F, t575: F, t687: F, t12587: F, t12622: F, t1611: F) -> (F, F, F, F, F, F) {
    let t38692 = F::cast_from(2.0_f64) * t1616 * t3909 * t2011;
    let t38699 = F::cast_from(6.0_f64) * t4915 * t3909 * t1617;
    let t38700 = t3903 * t1615;
    let t38702 = F::cast_from(2.0_f64) * t38700 * t1617;
    let t38703 = t12585 * t575;
    let t38705 = F::cast_from(2.0_f64) * t38703 * t687;
    let t38706 = t12587 * t2011;
    let t38708 = F::cast_from(2.0_f64) * t1611 * t12622;
    (t38692, t38699, t38702, t38705, t38706, t38708)
}
