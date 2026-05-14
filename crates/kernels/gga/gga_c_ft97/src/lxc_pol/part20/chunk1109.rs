//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1109/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1109<F: Float>(t2378: F, t6027: F, t24378: F, t27633: F, t27500: F, t108817: F, t109266: F, t109268: F, t109272: F, t109273: F, t109278: F, t109303: F, t109304: F, t109309: F, t13413: F, t13435: F, t13456: F, t13499: F, t13502: F, t13651: F, t18: F, t2409: F, t24337: F, t24361: F, t2455: F, t27487: F, t27651: F, t27652: F, t27729: F, t27733: F, t3766: F, t6018: F, t6035: F, t704: F, t709: F, t96716: F, t992: F) -> (F, F) {
    let t109310 = t6027 * t2378;
    let t109314 = t24378 * t27633;
    let t109316 = 0.56749874115226337448e-2 * t27500 * t109314;
    let t109317 = -2.0 * t27733 * t24337 - 2.0 * t3766 * t6018 * t13651 - 2.0 * t3766 * t27729 * t2455 + 0.17816121467177433866e-3 * t96716 * t109266 * t109268 + 0.59346127734643676855e-4 * t109272 * t109273 * t109268 + 0.32054706583615839486e-5 * t13413 * t109278 - 0.23238868087529279928e-3 * t27487 * t13456 + 0.77462893625097599762e-3 * t27487 * t13502 + 0.38731446812548799881e-3 * t27487 * t13499 - 0.64507906339763927061e-5 * t27487 * t13435 + 0.12768721675925925926e-1 * t24361 * t6035 * t704 * t992 * t2455 - 0.51074886703703703704e-1 * t24361 * t108817 * t704 * t18 * t709 + 0.25537443351851851852e-1 * t27651 * t6035 * t27652 * t2409 + 0.10338048737805743098e-4 * t109303 * t109304 * t109268 + 0.3443640424494650102e-5 * t109309 * t109310 * t109268 - t109316;
    (t109314, t109317)
}
