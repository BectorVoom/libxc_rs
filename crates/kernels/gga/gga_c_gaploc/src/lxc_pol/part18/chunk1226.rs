//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1226/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1226<F: Float>(t32033: F, t6963: F, t6964: F, t10526: F, t20471: F, t6540: F, t986: F, t1415: F, t1646: F, t2299: F, t2754: F, t10319: F, t4762: F, t10318: F, t4398: F, t26609: F, t6628: F) -> (F, F, F, F, F, F, F) {
    let t34592 = 0.85801175884441024006e1 * t6963 * t6964 * t32033;
    let t34595 = 0.42900587942220512002e1 * t20471 * t10526 * t32033;
    let t34600 = t6540 * t986;
    let t34603 = 0.71500979903700853338e0 * t1415 * t34600 * t1646;
    let t34604 = t2299 * t2754;
    let t34607 = 0.71500979903700853338e0 * t1415 * t34604 * t1646;
    let t34609 = 0.35750489951850426669e0 * t10319 * t4762;
    let t34612 = 0.71500979903700853338e0 * t4398 * t10318 * t1646;
    let t34614 = 0.21450293971110256002e1 * t26609 * t6628;
    (t34592, t34595, t34603, t34607, t34609, t34612, t34614)
}
