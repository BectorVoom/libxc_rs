//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1157/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1157<F: Float>(t35634: F, t35640: F, t35643: F, t35647: F, t35650: F, t35653: F, t35656: F, t35659: F, t35662: F, t35664: F, t35668: F, t35670: F, t35672: F, t35674: F, t35676: F, t35680: F, t35685: F, t35689: F, t35694: F, t35697: F, t35700: F, t35702: F, t35706: F, t35708: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t36423 = 0.86898242813537603825e-4 * t35634;
    let t36425 = 0.10862280351692200478e-4 * t35640;
    let t36426 = 0.64377114884362441502e-6 * t35643;
    let t36427 = 0.47522476538653377092e-5 * t35647;
    let t36428 = 0.47522476538653377092e-5 * t35650;
    let t36429 = 0.44241459320629195162e-6 * t35653;
    let t36430 = 0.17379648562707520765e-3 * t35656;
    let t36431 = 0.17379648562707520765e-3 * t35659;
    let t36432 = 0.809822844183586641e-4 * t35662;
    let t36433 = 0.28073858598364336888e-2 * t35664;
    let t36449 = 0.2429468532550759923e-3 * t35668 + 0.17379648562707520765e-3 * t35670 - 0.11948508386861420526e-3 * t35672 - 0.3090101514449397192e-4 * t35674 + 0.16871309253824721687e-5 * t35676 + 0.49207985323655438252e-6 * t35680 - 0.32292740368648881353e-6 * t35685 + 0.10862280351692200478e-4 * t35689 + 0.1030033838149799064e-5 * t35694 - 0.17379648562707520765e-4 * t35697 - 0.17379648562707520765e-4 * t35700 + 0.14420473734097186896e-3 * t35702 + 0.11446251026439642099e-6 * t35706 - 0.10527696974386626333e-2 * t35708;
    (t36423, t36425, t36426, t36427, t36428, t36429, t36430, t36431, t36432, t36433, t36449)
}
