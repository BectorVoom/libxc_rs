//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1129/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1129<F: Float>(t28097: F, t761: F, t766: F, t458: F, t6744: F, t6005: F, t1403: F, t27907: F, t681: F, t2354: F, t24204: F, t24240: F, t27894: F, t27939: F, t27976: F, t28006: F, t28010: F, t3746: F, t5996: F, t6011: F, t6068: F, t98208: F, t98211: F, t98214: F, t98219: F) -> (F, F) {
    let t109755 = t28097 * t761;
    let t109756 = t109755 * t766;
    let t109758 = t6744 * t458;
    let t109760 = t109758 * t6005 / 27.0;
    let t109767 = t1403 * t681 * t27907 / 9.0;
    let t109777 = -2.0 / 3.0 * t5996 * t27976 + 2.0 / 9.0 * t98208 + t98211 / 9.0 - t98214 / 18.0 - 4.0 * t109756 + t109760 + 2.0 / 9.0 * t28010 * t2354 * t24240 * t3746 - t109767 - t98219 / 9.0 + t5996 * t27939 / 3.0 - 2.0 / 3.0 * t27894 * t6011 + t27894 * t6068 / 3.0 - t24204 * t28006 / 9.0;
    (t109756, t109777)
}
