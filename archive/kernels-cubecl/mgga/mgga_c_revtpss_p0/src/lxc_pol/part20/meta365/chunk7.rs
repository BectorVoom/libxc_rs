//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1337/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1337<F: Float>(t39913: F, t39957: F, t40007: F, t40080: F, t158: F, t162: F, t2492: F, t9417: F, t9507: F, t760: F, t39791: F, t39795: F, t39799: F, t39807: F, t39813: F, t39818: F, t39823: F, t39857: F, t39859: F, t39861: F) -> (F, F, F, F, F) {
    let t40082 = t39913 + t39957 + t40007 + t40080;
    let t40084 = t158 * t162 * t40082;
    let t40086 = t9417 * t2492 * t9507;
    let t40088 = F::cast_from(0.62337092780453269531e3_f64) * t760 * t40086;
    let t40089 = -t39791 - t39795 + t39799 + t39807 - t39813 - t39818 - t39823 + t39857 + t39859 - t39861 + t40084 + t40088;
    (t40082, t40084, t40086, t40088, t40089)
}
