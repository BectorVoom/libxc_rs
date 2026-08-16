//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1147/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1147<F: Float>(t1907: F, t6781: F, t1868: F, t198: F, t21937: F, t22466: F, t22928: F, t22929: F, t22930: F, t22931: F, t22932: F, t4139: F, t532: F, t5532: F, t6816: F, t9542: F, t9593: F, t9598: F, t9854: F, t9857: F, t9865: F, t9868: F) -> (F, F) {
    let t23087 = t6781 * t1907;
    let t23092 = F::cast_from(2.0_f64) * t198 * t23087 * t532 * t9593 + F::cast_from(9.0_f64) * t1868 * t21937 * t4139 - F::cast_from(9.0_f64) * t1868 * t22466 * t4139 + F::cast_from(9.0_f64) * t4139 * t5532 * t6816 - t22928 + t22929 + t22930 + t22931 + t22932 + t9542 + t9598 - t9854 - t9857 + t9865 + t9868;
    (t23087, t23092)
}
