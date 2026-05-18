//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1326/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1326<F: Float>(t1415: F, t1646: F, t34600: F, t2299: F, t2754: F, t10319: F, t4762: F, t10318: F, t4398: F, t26609: F, t6628: F, t6798: F, t8411: F) -> (F, F, F, F, F, F) {
    let t34603 = F::new(0.71500979903700853338e0) * t1415 * t34600 * t1646;
    let t34604 = t2299 * t2754;
    let t34607 = F::new(0.71500979903700853338e0) * t1415 * t34604 * t1646;
    let t34609 = F::new(0.35750489951850426669e0) * t10319 * t4762;
    let t34612 = F::new(0.71500979903700853338e0) * t4398 * t10318 * t1646;
    let t34614 = F::new(0.21450293971110256002e1) * t26609 * t6628;
    let t34621 = F::new(0.14300195980740170668e1) * t8411 * t6798;
    (t34603, t34607, t34609, t34612, t34614, t34621)
}
