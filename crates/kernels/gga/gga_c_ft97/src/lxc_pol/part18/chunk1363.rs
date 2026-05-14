//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1363/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1363<F: Float>(t26860: F, t8392: F, t27330: F, t105376: F, t12968: F, t13054: F, t13088: F, t13167: F, t13208: F, t1901: F, t2142: F, t23455: F, t23470: F, t23510: F, t23572: F, t24009: F, t26849: F, t446: F, t50773: F, t574: F, t63052: F, t64242: F, t95469: F, t95471: F, t95477: F, t95487: F, t95492: F) -> (F,) {
    let t106361 = 4.0 / 27.0 * t8392 * t26860;
    let t106384 = 4.0 / 9.0 * t8392 * t27330;
    let t106392 = -2.0 / 3.0 * t1901 * t23470 * t13054 - t106361 - 2.0 / 9.0 * t1901 * t23470 * t13167 - t95469 / 9.0 - t95471 / 27.0 - 2.0 / 27.0 * t95477 + 2.0 / 3.0 * t446 * t574 * t2142 * t26849 + 2.0 / 27.0 * t95487 - 4.0 / 9.0 * t1901 * t63052 * t23510 - 2.0 / 9.0 * t1901 * t50773 * t24009 - 2.0 / 27.0 * t95492 + 2.0 / 3.0 * t1901 * t13208 * t105376 + t106384 + 4.0 / 3.0 * t1901 * t12968 * t23455 * t13088 - 4.0 / 3.0 * t1901 * t64242 * t23572;
    (t106392,)
}
