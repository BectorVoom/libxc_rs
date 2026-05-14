//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1267/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1267<F: Float>(t22914: F, t25564: F, t1286: F, t25848: F, t376: F, t102610: F, t103015: F, t1564: F, t1643: F, t1651: F, t1820: F, t22907: F, t22935: F, t25523: F, t25606: F, t26117: F, t26128: F, t379: F, t5501: F, t6421: F, t6547: F, t7793: F, t8418: F, t94081: F, t94104: F) -> (F,) {
    let t104006 = 2.0 / 3.0 * t22914 * t25564;
    let t104016 = t1286 * t376 * t25848 / 9.0;
    let t104017 = 2.0 / 9.0 * t5501 * t22907 * t26128 * t379 + 2.0 / 9.0 * t5501 * t22907 * t25523 * t379 + t5501 * t22907 * t6421 * t1651 / 9.0 + 4.0 * t103015 - t94081 / 18.0 - t5501 * t1564 * t26117 * t1651 / 18.0 - t5501 * t7793 * t26117 * t1643 / 27.0 - t104006 + 2.0 / 9.0 * t22935 * t25606 + 4.0 * t102610 + t94104 / 9.0 - 12.0 * t8418 * t6547 * t1820 - t104016;
    (t104017,)
}
