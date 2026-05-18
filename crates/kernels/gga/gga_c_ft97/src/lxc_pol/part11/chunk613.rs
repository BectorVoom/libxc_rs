//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 613/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk613<F: Float>(t1910: F, t8489: F, t1909: F, t1580: F, t432: F, t1903: F, t1902: F, t1913: F, t8392: F, t1820: F, t492: F, t1852: F) -> (F, F, F, F, F, F, F, F) {
    let t8490 = t1910 * t8489;
    let t8491 = t1909 * t8490;
    let t8494 = t1580 * t432;
    let t8495 = t1903 * t8494;
    let t8496 = t1902 * t8495;
    let t8499 = t8392 * t1913;
    let t8501 = t492 * t1820;
    let t8502 = t1852 * t8501;
    (t8490, t8491, t8494, t8495, t8496, t8499, t8501, t8502)
}
