//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta258 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta258<F: Float>(t1719: F, t3432: F, t1729: F, t2439: F, t1737: F, t3451: F, t3476: F, t3383: F, t1749: F, t3520: F, t3495: F, t1770: F, t3781: F) -> (F, F, F, F, F, F, F, F) {
        let (t16840, t16876, t17023, t17032, t17092, t17097, t17154, t17183) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1026::<F>(t1719, t3432, t1729, t2439, t1737, t3451, t3476, t3383, t1749, t3520, t3495, t1770, t3781);
    (t16840, t16876, t17023, t17032, t17092, t17097, t17154, t17183)
}
