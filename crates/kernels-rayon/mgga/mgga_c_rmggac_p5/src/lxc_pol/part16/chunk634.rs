//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 634/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk634(t5259: f64, t8901: f64, t4669: f64, t8905: f64, t2320: f64, t7414: f64, t1982: f64, t2314: f64, t7428: f64, t2191: f64, t2283: f64, t495: f64, t570: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9021 = t5259 * t8901;
    let t9023 = t4669 * t8905;
    let t9037 = t7414 * t2320;
    let t9040 = t2314 * t7428 * t1982;
    let t9042 = t2191 * t2283;
    let t9044 = t570 * t495;
    (t9021, t9023, t9037, t9040, t9042, t9044)
}
