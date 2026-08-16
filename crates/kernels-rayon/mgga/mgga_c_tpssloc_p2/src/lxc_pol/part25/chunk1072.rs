//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1072/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1072(t254: f64, t563: f64, t1878: f64, t22683: f64, t22844: f64, t6604: f64, t22759: f64, t242: f64, t1336: f64, t1887: f64, t22839: f64, t552: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26224 = t563 * t254;
    let t26284 = t1878 * t22683;
    let t26288 = t22844 * t6604;
    let t26308 = t22759 * t242;
    let t26309 = t1336 * t26308;
    let t26331 = t22839 * t1887;
    let t26446 = t6604 * t552;
    (t26224, t26284, t26288, t26309, t26331, t26446)
}
