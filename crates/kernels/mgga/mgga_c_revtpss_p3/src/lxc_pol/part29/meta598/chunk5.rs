//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2032/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2032<F: Float>(t102854: F, t103586: F, t14365: F, t14468: F, t18875: F, t1940: F, t2071: F, t2403: F, t2408: F, t2430: F, t26585: F, t26590: F, t27375: F, t27384: F, t28456: F, t28460: F, t4537: F, t51780: F, t61102: F, t61203: F, t63164: F, t7432: F, t775: F, t8020: F, t8031: F, t890: F, t95976: F, t98651: F, t98779: F) -> F {
    let t103706 = -F::new(2.0) * t102854 * t1940 * t890 + F::new(2.0) * t103586 * t1940 * t2408 - F::new(6.0) * t14365 * t2403 * t28460 + F::new(3.0) * t14468 * t2071 * t2403 - F::new(6.0) * t18875 * t2403 * t26585 - F::new(2.0) * t1940 * t26585 * t4537 + F::new(4.0) * t1940 * t26590 * t63164 + F::new(2.0) * t1940 * t26590 * t98779 + F::new(4.0) * t1940 * t27384 * t95976 + F::new(3.0) * t2403 * t2430 * t8020 - F::new(6.0) * t2403 * t26585 * t27375 + F::new(6.0) * t2403 * t28456 * t775 - F::new(6.0) * t2403 * t61102 * t7432 - F::new(3.0) * t2403 * t61203 * t7432 - F::new(3.0) * t2403 * t7432 * t98651 + F::new(6.0) * t51780 * t8031;
    t103706
}
