//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1193/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1193<F: Float>(t1000: F, t10615: F, t13632: F, t13912: F, t1435: F, t16887: F, t16891: F, t16895: F, t16919: F, t3608: F, t4038: F, t4054: F, t49707: F, t49754: F, t5065: F, t5069: F, t56939: F, t56941: F, t57046: F, t57628: F, t914: F, t999: F) -> (F,) {
    let t58027 = -4.0 * t13632 * t16919 + t56939 - 16.0 / 9.0 * t49707 - 8.0 * t4038 * t3608 * t10615 * t57628 + t56941 + 2.0 / 3.0 * t49754 * t1435 + 4.0 * t4054 * t16887 + t999 * t914 * t1000 * t57046 / 6.0 - 16.0 / 3.0 * t4054 * t16891 + 2.0 / 3.0 * t4054 * t16895 + t13912 * t5065 + 4.0 / 3.0 * t13912 * t5069;
    (t58027,)
}
