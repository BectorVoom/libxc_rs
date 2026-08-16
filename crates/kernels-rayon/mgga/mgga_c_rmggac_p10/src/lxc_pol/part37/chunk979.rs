//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 979/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk979(t77716: f64, t71340: f64, t8571: f64, t3219: f64, t9090: f64, t75662: f64, t75664: f64, t2144: f64, t3351: f64, t498: f64, t7231: f64, t9540: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t77717 = 0.42564599893297839398e-5_f64 * t77716;
    let t77718 = t8571 * t71340;
    let t77719 = 0.12769379967989351819e-4_f64 * t77718;
    let t77723 = t9090 * t3219;
    let t77724 = 0.99317399751028291929e-5_f64 * t77723;
    let t77725 = 0.3830813990396805546e-4_f64 * t75662;
    let t77726 = 0.1276937996798935182e-4_f64 * t75664;
    let t77732 = t3351 * t7231 * t2144 * t9540 * t498;
    (t77717, t77719, t77724, t77725, t77726, t77732)
}
