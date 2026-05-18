//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 820/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk820<F: Float>(t12837: F, t12878: F, t579: F, t91: F, t12306: F, t12308: F, t12310: F, t12285: F, t12290: F, t12293: F, t12296: F, t12300: F, t12304: F, t12315: F) -> (F, F) {
    let t12879 = t12837 + t12878;
    let t12881 = t91 * t579 * t12879;
    let t12889 = F::new(2.0) / F::new(27.0) * t12306;
    let t12890 = F::new(4.0) / F::new(27.0) * t12308;
    let t12891 = F::new(4.0) / F::new(81.0) * t12310;
    let t12893 = t12881 / F::new(6.0) + t12285 / F::new(9.0) + F::new(2.0) / F::new(27.0) * t12290 - F::new(10.0) / F::new(81.0) * t12293 - F::new(8.0) / F::new(27.0) * t12296 + t12300 / F::new(9.0) + F::new(4.0) / F::new(9.0) * t12304 - t12889 - t12890 + t12891 - F::new(2.0) / F::new(9.0) * t12315;
    (t12881, t12893)
}
