//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1790/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1790<F: Float>(t9590: F, t9593: F, t1353: F, t13625: F, t25802: F, t3829: F, t3889: F, t39773: F, t39783: F, t39786: F, t39791: F, t39795: F, t4139: F, t47006: F, t47008: F, t47010: F, t47012: F, t47014: F, t47017: F, t5536: F, t9599: F) -> F {
    let t47638 = t9590 * t9593;
    let t47648 = F::new(24.0) * t1353 * t4139 * t47638 - F::new(36.0) * t13625 * t25802 * t4139 - F::new(36.0) * t3829 * t5536 * t9599 - F::new(18.0) * t3889 * t4139 * t9599 + t39773 - t39783 - t39786 - t39791 - t39795 + t47006 - t47008 + t47010 - t47012 + t47014 + t47017;
    t47648
}
