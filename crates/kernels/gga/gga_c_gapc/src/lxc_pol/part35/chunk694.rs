//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 694/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk694<F: Float>(t2973: F, t8801: F, t2979: F, t5987: F, t2983: F, t1: F, t118: F, t3: F, t169: F, t173: F, t1944: F, t3079: F, t563: F, t1018: F, t458: F, t1012: F, t1386: F) -> (F, F, F, F, F) {
    let t8802 = t2973 * t8801;
    let t8804 = t5987 * t2979;
    let t8805 = t8804 * t2983;
    let t8808 = t118 * t1 * t3;
    let t8809 = t169 * t8808;
    let t8810 = t1944 * t173;
    let t8811 = t8809 * t8810;
    let t8813 = t563 * t3079;
    let t8814 = t1018 * t458;
    let t8815 = t8813 * t8814;
    let t8817 = t1386 * t1012;
    (t8802, t8805, t8811, t8815, t8817)
}
