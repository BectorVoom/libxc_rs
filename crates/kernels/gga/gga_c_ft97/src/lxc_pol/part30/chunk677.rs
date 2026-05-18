//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 677/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk677<F: Float>(t1486: F, t681: F, t7071: F, t10248: F, t28760: F, t446: F, t1234: F, t6260: F, t852: F, t193: F, t6308: F, t1476: F, t4226: F) -> (F, F, F, F, F, F) {
    let t28811 = t1486 * t681 * t7071;
    let t28813 = t10248 * t28760;
    let t28814 = t446 * t28813;
    let t28816 = t6260 * t1234;
    let t28817 = t852 * t28816;
    let t28819 = t6308 * t193 * t28817;
    let t28821 = t1476 * t4226;
    (t28811, t28813, t28814, t28816, t28819, t28821)
}
