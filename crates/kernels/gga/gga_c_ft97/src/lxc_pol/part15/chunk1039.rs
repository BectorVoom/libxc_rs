//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1039/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1039<F: Float>(t10261: F, t10613: F, t192: F, t19709: F, t22161: F, t2681: F, t2766: F, t2771: F, t2781: F, t4199: F, t4206: F, t4218: F, t43803: F, t462: F, t5299: F, t83569: F, t83587: F, t848: F, t852: F, t88149: F, t88153: F, t88184: F, t88240: F, t88248: F, t89770: F, t89889: F, t90304: F, t90308: F, t90313: F, t92: F) -> (F,) {
    let t90464 = 4.0 / 3.0 * t462 * t2771 * t89770 + 8.0 / 3.0 * t462 * t4206 * t88149 - 8.0 / 9.0 * t462 * t4199 * t88153 + 8.0 * t462 * t4199 * t88184 - 16.0 / 3.0 * t462 * t10613 * t89889 + 8.0 * t462 * t2681 * t4218 * t22161 - 2.0 / 3.0 * t462 * t2766 * t88240 - t462 * t848 * t88248 / 3.0 - 36.0 * t462 * t10261 * t19709 * t5299 - 16.0 / 9.0 * t83569 + 6.0 * t92 * t192 * t2781 * t90313 + 24.0 * t92 * t192 * t43803 * t90308 - t92 * t192 * t852 * t90304 - 8.0 * t83587;
    (t90464,)
}
