//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1076/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1076<F: Float>(t16296: F, t2297: F, t4818: F, t16288: F, t16292: F, t16294: F, t16300: F, t16304: F, t21737: F, t21740: F, t21743: F, t21745: F, t21747: F, t21751: F, t21755: F, t21759: F) -> (F, F) {
    let t21762 = t16296 * t2297 * t4818;
    let t21769 = -0.7335e0 * t21737 + 0.489e0 * t21740 + 0.2445e0 * t21743 - 0.489e0 * t21745 + 0.2445e0 * t21747 + 0.2445e0 * t21751 - 0.12225e0 * t21755 - 0.12225e0 * t21759 - 0.8802e1 * t21762 + 0.1956e1 * t16288 - 0.489e0 * t16292 - 0.21733333333333333333e1 * t16294 + 0.978e0 * t16300 - 0.12225e0 * t16304;
    (t21762, t21769)
}
