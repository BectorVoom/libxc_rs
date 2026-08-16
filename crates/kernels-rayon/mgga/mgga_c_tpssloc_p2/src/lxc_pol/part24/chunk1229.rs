//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1229/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1229(t25796: f64, t6690: f64, t28: f64, t870: f64, t10143: f64, t1868: f64, t671: f64, t1982: f64, t8944: f64, t12461: f64, t2018: f64, t254: f64, t563: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25797 = t25796 * t6690;
    let t25891 = t870 * t28;
    let t25927 = t10143 * t28;
    let t26103 = t1868 * t671;
    let t26161 = t1982 * t8944;
    let t26162 = t2018 * t12461;
    let t26224 = t563 * t254;
    (t25797, t25891, t25927, t26103, t26161, t26162, t26224)
}
