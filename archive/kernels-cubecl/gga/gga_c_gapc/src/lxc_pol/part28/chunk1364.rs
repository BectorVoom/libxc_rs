//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1364/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1364<F: Float>(t35668: F, t35670: F, t35672: F, t35674: F, t35676: F, t35680: F, t35685: F, t35689: F, t35694: F, t35697: F, t35700: F, t35702: F, t35706: F, t35708: F) -> F {
    let t36449 = F::cast_from(0.2429468532550759923e-3_f64) * t35668 + F::cast_from(0.17379648562707520765e-3_f64) * t35670 - F::cast_from(0.11948508386861420526e-3_f64) * t35672 - F::cast_from(0.3090101514449397192e-4_f64) * t35674 + F::cast_from(0.16871309253824721687e-5_f64) * t35676 + F::cast_from(0.49207985323655438252e-6_f64) * t35680 - F::cast_from(0.32292740368648881353e-6_f64) * t35685 + F::cast_from(0.10862280351692200478e-4_f64) * t35689 + F::cast_from(0.1030033838149799064e-5_f64) * t35694 - F::cast_from(0.17379648562707520765e-4_f64) * t35697 - F::cast_from(0.17379648562707520765e-4_f64) * t35700 + F::cast_from(0.14420473734097186896e-3_f64) * t35702 + F::cast_from(0.11446251026439642099e-6_f64) * t35706 - F::cast_from(0.10527696974386626333e-2_f64) * t35708;
    t36449
}
