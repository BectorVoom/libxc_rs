//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 741/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk741<F: Float>(t169: F, t8808: F, t173: F, t1944: F, t3079: F, t563: F, t1018: F, t458: F, t1012: F, t1386: F, t182: F, t1033: F, t128: F) -> (F, F, F, F) {
    let t8809 = t169 * t8808;
    let t8810 = t1944 * t173;
    let t8811 = t8809 * t8810;
    let t8813 = t563 * t3079;
    let t8814 = t1018 * t458;
    let t8815 = t8813 * t8814;
    let t8817 = t1386 * t1012;
    let t8818 = t8817 * t182;
    let t8820 = t128 * t1033;
    (t8811, t8815, t8818, t8820)
}
