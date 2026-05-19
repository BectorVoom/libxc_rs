//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 907/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk907<F: Float>(t3730: F, t824: F, t218: F, t219: F, t334: F, t9795: F, t6175: F, t6177: F, t7950: F, t7980: F, t7983: F, t9812: F, t9814: F, t9819: F, t9823: F, t9826: F) -> (F, F, F, F, F) {
    let t9828 = t824 * t3730;
    let t9830 = t218 * t219 * t9828;
    let t9832 = t334 * t9795;
    let t9834 = t218 * t219 * t9832;
    let t9836 = F::new(0.15358125e0) * t9812 + F::new(0.3071625e0) * t9814 - t6175 + F::cast_from(0.27385555555555555556e0_f64) * t6177 + F::cast_from(0.5477111111111111111e0_f64) * t7950 - t7980 - t7983 - F::cast_from(0.16431333333333333333e0_f64) * t9819 + F::new(0.49294e0) * t9823 - F::cast_from(0.16431333333333333333e0_f64) * t9826 + F::new(0.24647e0) * t9830 + F::new(0.24647e0) * t9834;
    (t9828, t9830, t9832, t9834, t9836)
}
