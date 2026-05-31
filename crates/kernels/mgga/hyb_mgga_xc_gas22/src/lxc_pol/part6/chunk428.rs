//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 428/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk428<F: Float>(t43: F, t1897: F, t39: F, t1808: F, t1895: F, t1802: F, t575: F, t1796: F, t578: F, t1888: F, t1891: F, t572: F) -> (F, F, F, F, F, F, F) {
    let t45 = F::cast_from(0.135e1_f64) < t43;
    let t1898 = t39 * t1897;
    let t1900 = t1895 * t1898 * t1808;
    let t1903 = t39 * t1802;
    let t1905 = t575 * t1903 * t1808;
    let t1909 = t575 * t578 * t1796;
    let t1912 = t1888 + t1891 / F::cast_from(81.0_f64) - t572 * t1900 / F::cast_from(81.0_f64) + t572 * t1905 / F::cast_from(27.0_f64) - t572 * t1909 / F::cast_from(54.0_f64);
    let t1913 = piecewise3::<F>(t45, t1912, F::cast_from(0.0_f64));
    (t1898, t1900, t1903, t1905, t1909, t1912, t1913)
}
