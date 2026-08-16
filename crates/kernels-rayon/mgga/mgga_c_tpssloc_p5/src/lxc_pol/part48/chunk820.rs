//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 820/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk820(t254: f64, t563: f64, t12020: f64, t2015: f64, t1887: f64, t22839: f64, t12461: f64, t2094: f64, t193: f64, t200: f64, t2056: f64, t10109: f64, t2053: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26224 = t563 * t254;
    let t26225 = t12020 * t2015;
    let t26331 = t22839 * t1887;
    let t26558 = t2094 * t12461;
    let t26563 = t193 * t200 * t2056;
    let t26728 = t10109 * t2053;
    (t26224, t26225, t26331, t26558, t26563, t26728)
}
