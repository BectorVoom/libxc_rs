//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 749/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk749(t169: f64, t8808: f64, t173: f64, t1944: f64, t3079: f64, t563: f64, t1018: f64, t458: f64, t1012: f64, t1386: f64, t182: f64, t1033: f64, t128: f64) -> (f64, f64, f64, f64) {
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
