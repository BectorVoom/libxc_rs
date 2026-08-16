//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1272/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1272(t100784: f64, t7788: f64, t19317: f64, t303: f64, t356: f64, t27924: f64, t5019: f64, t1020: f64, t6620: f64, t92701: f64, t26671: f64, t6625: f64) -> (f64, f64, f64, f64, f64) {
    let t100805 = t7788 * t100784;
    let t100814 = t303 * t356 * t19317;
    let t100817 = t303 * t27924 * t5019;
    let t100820 = t1020 * t92701 * t6620;
    let t100823 = t1020 * t26671 * t6625;
    (t100805, t100814, t100817, t100820, t100823)
}
