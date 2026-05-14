//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 760/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk760<F: Float>(t1901: F, t33630: F, t33636: F, t35555: F, t35559: F, t35563: F, t35567: F, t35570: F, t35574: F, t35578: F, t35582: F, t35586: F, t35590: F, t446: F, t1168: F, t7484: F) -> (F, F) {
    let t35593 = 2.0 / 3.0 * t446 * t35555 - 2.0 / 9.0 * t1901 * t35559 + t1901 * t35563 / 9.0 - 2.0 / 9.0 * t1901 * t35567 + 2.0 / 9.0 * t1901 * t35570 + 4.0 / 3.0 * t446 * t35574 + 2.0 / 3.0 * t446 * t35578 - t33630 + 2.0 / 3.0 * t446 * t35582 + 2.0 / 3.0 * t446 * t35586 + t33636 - t446 * t35590 / 9.0;
    let t35594 = t7484 * t1168;
    (t35593, t35594)
}
