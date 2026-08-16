//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 819/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk819(t1954: f64, t709: f64, t5484: f64, t722: f64, t1916: f64, t1918: f64, t1938: f64, t1950: f64, t1955: f64, t1957: f64, t1972: f64, t1977: f64, t5736: f64, t5740: f64, t5768: f64, t5770: f64, t5773: f64, t5779: f64, t5799: f64, t5807: f64, t5883: f64, t5887: f64, t5890: f64, t5894: f64, t5897: f64, t5900: f64) -> (f64, f64, f64) {
    let t5903 = t709 * t1954;
    let t5906 = t5484 * t722;
    let t5909 = 0.17544670867903938621e1_f64 * t1950 * t1972 - 6.0_f64 * t1916 * t5883 + 0.96491876992155210402e2_f64 * t1938 * t5887 - 0.35089341735807877242e1_f64 * t1955 * t5890 + 0.51947577317044391277e2_f64 * t1977 * t5894 - t5770 - t5773 + t5779 - t5799 - t5807 - 6.0_f64 * t5897 * t1918 + 6.0_f64 * t1938 * t5900 - 0.35089341735807877242e1_f64 * t5903 * t1957 + 0.35089341735807877242e1_f64 * t1977 * t5906 - t5768 + t5736 - t5740;
    (t5903, t5906, t5909)
}
