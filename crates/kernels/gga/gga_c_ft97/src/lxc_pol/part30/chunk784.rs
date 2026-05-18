//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 784/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk784<F: Float>(t2035: F, t7590: F, t820: F, t230: F, t811: F, t420: F, t7470: F, t33411: F, t7006: F, t291: F, t4092: F, t52: F, t7457: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33889 = t2035 * t7590 * t820;
    let t33892 = t230 * t811;
    let t33893 = t420 * t33892;
    let t33894 = t7470 * t33893;
    let t33897 = t230 * t820;
    let t33898 = t420 * t33897;
    let t33899 = t7470 * t33898;
    let t33903 = F::new(0.30209702213418583705e-1) * t7006 * t33411;
    let t33906 = t4092 * t291;
    let t33908 = t52 * t7457 * t811;
    (t33889, t33892, t33893, t33894, t33897, t33898, t33899, t33903, t33906, t33908)
}
