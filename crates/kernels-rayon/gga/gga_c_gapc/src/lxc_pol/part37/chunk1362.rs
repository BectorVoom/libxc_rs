//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1362/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1362(t35662: f64, t35664: f64, t35668: f64, t35670: f64, t35672: f64, t35674: f64, t35676: f64, t35680: f64, t35685: f64, t35689: f64, t35694: f64, t35697: f64, t35700: f64, t35702: f64, t35706: f64, t35708: f64) -> (f64, f64, f64) {
    let t36432 = 0.809822844183586641e-4_f64 * t35662;
    let t36433 = 0.28073858598364336888e-2_f64 * t35664;
    let t36449 = 0.2429468532550759923e-3_f64 * t35668 + 0.17379648562707520765e-3_f64 * t35670 - 0.11948508386861420526e-3_f64 * t35672 - 0.3090101514449397192e-4_f64 * t35674 + 0.16871309253824721687e-5_f64 * t35676 + 0.49207985323655438252e-6_f64 * t35680 - 0.32292740368648881353e-6_f64 * t35685 + 0.10862280351692200478e-4_f64 * t35689 + 0.1030033838149799064e-5_f64 * t35694 - 0.17379648562707520765e-4_f64 * t35697 - 0.17379648562707520765e-4_f64 * t35700 + 0.14420473734097186896e-3_f64 * t35702 + 0.11446251026439642099e-6_f64 * t35706 - 0.10527696974386626333e-2_f64 * t35708;
    (t36432, t36433, t36449)
}
