//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 628/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk628<F: Float>(t422: F, t4896: F, t1815: F, t639: F, t1733: F, t626: F, t1809: F, t1620: F, t1775: F, t583: F, t220: F, t2735: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4897 = t4896 * t422;
    let t4898 = t1815 * t4897;
    let t4900 = F::new(4.0) / F::new(15.0) * t639 * t4898;
    let t4901 = t1733 * t626;
    let t4902 = t4901 * t422;
    let t4903 = t1809 * t4902;
    let t4905 = F::new(8.0) / F::new(15.0) * t1620 * t4903;
    let t4906 = t1775 * t583;
    let t4907 = F::new(4.0) / F::new(15.0) * t4906;
    let t4908 = t2735 * t220;
    (t4897, t4898, t4900, t4901, t4902, t4903, t4905, t4907, t4908)
}
