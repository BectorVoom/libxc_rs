//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2188/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2188<F: Float>(t1113: F, t4343: F, t1583: F, t3351: F, t27799: F, t63164: F, t100975: F, t100978: F, t100982: F, t100988: F, t100993: F, t1940: F, t1963: F, t2403: F, t25206: F, t25440: F, t25752: F, t25760: F, t25784: F, t27368: F, t27382: F, t27770: F, t27793: F, t27806: F, t4541: F, t7091: F, t7783: F, t7869: F, t92775: F, t92819: F, t98637: F) -> F {
    let t100997 = t1113 * t4343;
    let t101012 = t3351 * t1583;
    let t101016 = t27799 * t63164;
    let t101021 = F::new(2.0) * t27382 * t100975 + F::new(6.0) * t25206 * t100978 - F::new(3.0) * t27382 * t100982 - F::new(3.0) * t92819 * t27793 - F::new(3.0) * t25206 * t100988 - t1940 * t25440 * t27806 + F::new(3.0) / F::new(2.0) * t2403 * t1963 * t100993 + F::new(3.0) * t2403 * t1963 * t100997 - t1940 * t27368 * t25784 / F::new(2.0) - F::new(3.0) * t92819 * t27770 + F::new(3.0) * t4541 * t7783 * t25752 - t1940 * t92775 * t7869 / F::new(2.0) - t1940 * t7091 * t101012 / F::new(2.0) + F::new(2.0) * t27382 * t101016 - F::new(3.0) * t98637 * t25760;
    t101021
}
