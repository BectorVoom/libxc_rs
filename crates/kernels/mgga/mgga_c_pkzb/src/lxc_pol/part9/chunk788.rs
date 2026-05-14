//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 788/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk788<F: Float>(t1916: F, t1918: F, t1938: F, t1950: F, t1955: F, t1957: F, t1972: F, t1977: F, t5736: F, t5740: F, t5768: F, t5770: F, t5773: F, t5779: F, t5799: F, t5807: F, t5883: F, t5887: F, t5890: F, t5894: F, t5897: F, t5900: F, t5903: F, t5906: F) -> (F,) {
    let t5909 = 0.17544670867903938621e1 * t1950 * t1972 - 6.0 * t1916 * t5883 + 0.96491876992155210402e2 * t1938 * t5887 - 0.35089341735807877242e1 * t1955 * t5890 + 0.51947577317044391277e2 * t1977 * t5894 - t5770 - t5773 + t5779 - t5799 - t5807 - 6.0 * t5897 * t1918 + 6.0 * t1938 * t5900 - 0.35089341735807877242e1 * t5903 * t1957 + 0.35089341735807877242e1 * t1977 * t5906 - t5768 + t5736 - t5740;
    (t5909,)
}
