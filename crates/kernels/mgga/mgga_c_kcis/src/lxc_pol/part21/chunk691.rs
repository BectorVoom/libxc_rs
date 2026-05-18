//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 691/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk691<F: Float>(t2165: F, t228: F, t2766: F, t2771: F, t7608: F, t7610: F, t7611: F, t7614: F, t7631: F, t7655: F, t7657: F, t7660: F, t7669: F, t899: F, t906: F) -> F {
    let t7671 = -t2165 * t2766 + t228 * t7655 + F::new(2.0) * t2771 * t7660 - t7657 * t906 - t7669 * t899 - t7608 + t7610 + t7611 - t7614 + t7631;
    t7671
}
