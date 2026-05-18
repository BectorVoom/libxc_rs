//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1275/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1275<F: Float>(t24321: F, t787: F, t9824: F, t10677: F, t10831: F, t10954: F, t1445: F, t1865: F, t28281: F, t28284: F, t32984: F, t32987: F, t32991: F, t32997: F, t33001: F, t33004: F, t33009: F, t33013: F, t33018: F, t33021: F, t4614: F, t5676: F, t813: F) -> F {
    let t33023 = t787 * t24321 * t9824;
    let t33024 = F::new(0.14896037479937677779e-1) * t33023;
    let t33025 = -t32984 - t32987 + t32991 - F::new(0.92023022289409799224e1) * t813 * t1445 * t10677 * t1865 + t32997 - t33001 + t33004 - F::new(0.12269736305254639896e2) * t813 * t4614 * t10954 + t28281 - t33009 - t33013 + F::new(0.79445533226334281486e-1) * t5676 * t10831 + t33018 + t33021 + t33024 - t28284;
    t33025
}
