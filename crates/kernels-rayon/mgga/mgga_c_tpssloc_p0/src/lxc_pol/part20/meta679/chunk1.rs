//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2564/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2564(t14858: f64, t3423: f64, t51839: f64, t51844: f64, t51847: f64, t51851: f64, t51853: f64, t51855: f64, t51857: f64, t51859: f64, t51862: f64, t51864: f64) -> (f64, f64) {
    let t51866 = 0.51947577317044391276e2_f64 * t14858 * t3423;
    let t51867 = -t51839 - t51844 + t51847 + t51851 + t51853 + t51855 + t51857 + t51859 + t51862 - t51864 - t51866;
    (t51866, t51867)
}
