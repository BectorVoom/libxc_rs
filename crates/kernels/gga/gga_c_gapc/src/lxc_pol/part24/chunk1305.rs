//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1305/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1305<F: Float>(t34066: F, t34069: F, t34071: F, t34075: F, t34079: F, t34084: F, t34088: F, t34092: F, t34095: F, t34098: F, t34100: F, t34104: F, t34108: F, t34111: F, t34114: F, t34117: F, t34119: F, t34121: F, t34125: F, t34127: F, t34132: F, t34135: F) -> (F, F) {
    let t37989 = -F::cast_from(0.13505639832369200846e-5_f64) * t34066 - F::cast_from(0.8004342540650813035e-7_f64) * t34069 - F::cast_from(0.80189736504692130024e-6_f64) * t34071 - F::cast_from(0.5238829942192678162e-8_f64) * t34075 - F::cast_from(0.64454108540756375024e-8_f64) * t34079 + F::cast_from(0.12144531250000000001e-2_f64) * t34084 + F::cast_from(0.17678841199750320007e-7_f64) * t34088 - F::cast_from(0.19676021349741883234e-7_f64) * t34092 - F::cast_from(0.13505639832369200846e-5_f64) * t34095 + F::cast_from(0.15716489826578034486e-7_f64) * t34098 - F::cast_from(0.7246363367825880434e-6_f64) * t34100;
    let t38001 = F::cast_from(0.9275345110817126956e-4_f64) * t34104 + F::cast_from(0.22544241588791628019e-6_f64) * t34108 + F::cast_from(0.13900948042322754167e-2_f64) * t34111 - F::cast_from(0.98326426188151041676e-7_f64) * t34114 + F::cast_from(0.49163213094075520838e-8_f64) * t34117 - F::cast_from(0.14068374825384584215e-7_f64) * t34119 - F::cast_from(0.14068374825384584215e-7_f64) * t34121 + F::cast_from(0.19191204183684243232e-6_f64) * t34125 + F::cast_from(0.68358185972367904025e-5_f64) * t34127 - F::cast_from(0.49163213094075520838e-8_f64) * t34132 + F::cast_from(0.5060221354166666667e-5_f64) * t34135;
    (t37989, t38001)
}
