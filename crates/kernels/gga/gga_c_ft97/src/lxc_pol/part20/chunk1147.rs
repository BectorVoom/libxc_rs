//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1147/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1147<F: Float>(t24407: F, t51340: F, t10007: F, t107794: F, t107891: F, t109609: F, t11593: F, t13830: F, t13853: F, t13885: F, t14121: F, t14127: F, t14226: F, t14245: F, t1901: F, t242: F, t24668: F, t2574: F, t28128: F, t28246: F, t28404: F, t3837: F, t446: F, t6166: F, t6194: F, t684: F, t729: F, t97517: F, t97526: F, t97528: F, t97559: F, t97793: F) -> (F, F) {
    let t110323 = t51340 * t24407;
    let t110361 = 4.0 / 3.0 * t1901 * t13885 * t28128 * t14226 - t97517 / 9.0 - 2.0 * t446 * t242 * t110323 + 2.0 / 3.0 * t446 * t242 * t107794 + 4.0 / 3.0 * t446 * t242 * t107891 - t97526 / 9.0 - 2.0 / 27.0 * t97528 - 4.0 / 3.0 * t1901 * t14127 * t24668 * t14245 + 8.0 / 27.0 * t11593 * t28404 * t14121 - 2.0 / 9.0 * t1901 * t10007 * t28246 * t684 + 4.0 / 3.0 * t446 * t2574 * t6194 * t3837 - 2.0 * t446 * t242 * t109609 + 2.0 / 3.0 * t446 * t729 * t13830 * t6166 - 2.0 / 9.0 * t1901 * t97793 * t13853 + 4.0 / 27.0 * t97559;
    (t110323, t110361)
}
