//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 768/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk768(t2078: f64, t3851: f64, t7834: f64, t797: f64, t321: f64, t7840: f64, t5259: f64, t333: f64, t4669: f64, t128: f64, t305: f64, t3899: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35815 = t3851 * t2078;
    let t35824 = t797 * t7834;
    let t35844 = t7840 * t321;
    let t35845 = t5259 * t35844;
    let t35847 = t7840 * t333;
    let t35848 = t4669 * t35847;
    let t35861 = t305 * t128 * t3899;
    (t35815, t35824, t35844, t35845, t35847, t35848, t35861)
}
