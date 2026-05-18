//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1256/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1256<F: Float>(t34222: F, t686: F, t72: F, t32705: F, t32710: F, t125617: F, t121000: F, t121004: F, t122321: F, t122327: F, t122331: F, t1444: F, t32250: F, t32700: F, t34223: F, t34227: F, t7506: F, t7910: F, t8706: F, t8707: F) -> F {
    let t128628 = t34222 * t72 * t686;
    let t128629 = t32705 * t128628;
    let t128631 = t32710 * t128628;
    let t128644 = F::new(0.263521689745817692e-2) * t125617;
    let t128647 = t122321 + F::new(0.26447628533477078895e-3) * t121000 + t121004 - F::new(0.14279934416275588154e-1) * t128629 + F::new(0.25389723392137995738e-1) * t128631 + F::new(0.14456046980341999104e-1) * t122327 + t122331 + F::new(0.57119737665102352616e0) * t32700 * t34227 + F::new(0.57119737665102352616e0) * t8706 * t8707 * t7506 * t7910 - F::new(0.17135921299530705785e1) * t8706 * t32250 * t34222 * t1444 + t128644 + F::new(0.57119737665102352616e0) * t32700 * t34223;
    t128647
}
