//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 355/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk355<F: Float>(t363: F, t5691: F, t1564: F, t446: F, t432: F, t5507: F, t28: F, t89: F, t370: F, t5617: F, t27: F, t5669: F, t5673: F, t5678: F, t5682: F, t5686: F, t5690: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5692 = t5691 * t363;
    let t5693 = t1564 * t5692;
    let t5694 = t446 * t5693;
    let t5696 = t5507 * t432;
    let t5697 = t28 * t5696;
    let t5698 = t89 * t5697;
    let t5700 = t370 * t5617;
    let t5702 = t89 * t27 * t5700;
    let t5704 = t5669 / 12.0 + t5673 + t5678 / 18.0 + t5682 / 3.0 - t5686 / 6.0 + t5690 + t5694 / 9.0 + 2.0 / 3.0 * t5698 - t5702 / 3.0;
    (t5692, t5693, t5694, t5696, t5697, t5698, t5700, t5702, t5704)
}
