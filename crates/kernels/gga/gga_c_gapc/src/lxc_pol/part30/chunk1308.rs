//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1308/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1308<F: Float>(t34066: F, t34069: F, t34071: F, t34075: F, t34079: F, t34084: F, t34088: F, t34092: F, t34095: F, t34098: F, t34100: F, t34104: F, t34108: F, t34111: F, t34114: F, t34117: F, t34119: F, t34121: F, t34125: F, t34127: F, t34132: F, t34135: F) -> (F, F) {
    let t37989 = -F::new(0.13505639832369200846e-5) * t34066 - F::new(0.8004342540650813035e-7) * t34069 - F::new(0.80189736504692130024e-6) * t34071 - F::new(0.5238829942192678162e-8) * t34075 - F::new(0.64454108540756375024e-8) * t34079 + F::new(0.12144531250000000001e-2) * t34084 + F::new(0.17678841199750320007e-7) * t34088 - F::new(0.19676021349741883234e-7) * t34092 - F::new(0.13505639832369200846e-5) * t34095 + F::new(0.15716489826578034486e-7) * t34098 - F::new(0.7246363367825880434e-6) * t34100;
    let t38001 = F::new(0.9275345110817126956e-4) * t34104 + F::new(0.22544241588791628019e-6) * t34108 + F::new(0.13900948042322754167e-2) * t34111 - F::new(0.98326426188151041676e-7) * t34114 + F::new(0.49163213094075520838e-8) * t34117 - F::new(0.14068374825384584215e-7) * t34119 - F::new(0.14068374825384584215e-7) * t34121 + F::new(0.19191204183684243232e-6) * t34125 + F::new(0.68358185972367904025e-5) * t34127 - F::new(0.49163213094075520838e-8) * t34132 + F::new(0.5060221354166666667e-5) * t34135;
    (t37989, t38001)
}
