//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1060/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1060<F: Float>(t23054: F, t34394: F, t2: F, t34482: F, t1564: F, t379: F, t5674: F, t1882: F, t34500: F, t1317: F, t145585: F, t28: F, t469: F) -> (F, F, F, F) {
    let t145621 = t23054 * t34394;
    let t145623 = t2 * t34482;
    let t145626 = t5674 * t1564 * t145623 * t379;
    let t145628 = t1882 * t34500;
    let t145632 = t1317 * t28 * t469 * t145585;
    (t145621, t145626, t145628, t145632)
}
