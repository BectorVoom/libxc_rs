//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1191/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1191(t11783: f64, t11784: f64, t11785: f64, t11786: f64, t11790: f64, t11793: f64, t11795: f64, t11802: f64, t11804: f64, t11805: f64, t11806: f64, t11807: f64, t11808: f64, t11815: f64, t11816: f64, t11820: f64, t11823: f64, t11825: f64, t11827: f64, t11829: f64, t11831: f64, t11833: f64, t11835: f64) -> (f64, f64) {
    let t14321 = -t11783 - t11784 + t11785 + t11786 - t11790 - t11793 - t11795 + t11802 - t11804 - t11805 - t11806;
    let t14322 = t11807 - t11808 + t11815 - t11816 + t11820 + t11823 + t11825 + t11827 + t11829 + t11831 + t11833 - t11835;
    (t14321, t14322)
}
