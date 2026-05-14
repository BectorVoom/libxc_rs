//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 960/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk960<F: Float>(t4698: F, t72: F, t5579: F, t4710: F, t26695: F, t925: F, t26721: F, t5570: F, t1013: F, t26743: F, t22591: F, t1701: F, t4702: F, t5546: F, t23705: F, t23715: F, t23847: F, t23869: F, t23877: F, t26631: F, t26658: F, t29559: F, t5813: F, t5829: F, t5838: F, t8833: F, t8838: F) -> (F, F, F, F, F, F, F, F) {
    let t30071 = t72 * t4698;
    let t30072 = t5579 * t30071;
    let t30075 = t72 * t4710;
    let t30079 = t26695 * t925;
    let t30083 = t26721 * t925;
    let t30084 = t5570 * t30083;
    let t30090 = t26743 * t1013;
    let t30091 = t22591 * t30090;
    let t30095 = t1701 * t5546 * t4702;
    let t30103 = -0.10001700163888888889e0 * t5813 * t30072 + 0.10001700163888888889e0 * t5829 * t5579 * t30075 + 0.66678001092592592595e-1 * t23705 * t5570 * t30079 - 0.66678001092592592595e-1 * t23715 * t30084 - 0.11113000182098765433e-1 * t26631 + t23877 + 0.33339000546296296298e-1 * t5838 * t29559 - 0.90613700826057446696e0 * t23847 * t30091 + 0.45306850413028723348e0 * t8838 * t30095 + 0.90613700826057446696e0 * t23869 * t30091 - 0.45306850413028723348e0 * t8833 * t30095 + 0.66678001092592592595e-1 * t26658;
    (t30071, t30072, t30075, t30079, t30083, t30084, t30091, t30103)
}
