//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1083/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1083<F: Float>(t783: F, t91827: F, t91861: F, t2538: F, t26651: F, t826: F, t2153: F, t35630: F, t26416: F, t8522: F, t2626: F, t26516: F, t7630: F, t9268: F, t35635: F, t9276: F) -> (F, F, F, F, F, F, F) {
    let t91863 = t783 * (t91827 + t91861);
    let t91866 = 6.0 * t2538 * t26651 * t826;
    let t91869 = t35630 * t2153;
    let t91872 = 6.0 * t8522 * t26416;
    let t91874 = 3.0 * t26516 * t2626;
    let t91885 = 3.0 * t9268 * t7630;
    let t91895 = 24.0 * t35635 * t2153 * t9276;
    (t91863, t91866, t91869, t91872, t91874, t91885, t91895)
}
