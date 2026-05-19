//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1147/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1147<F: Float>(t14188: F, t869: F, t689: F, t225: F, t9990: F, t213: F, t10062: F, t10130: F, t13805: F, t1399: F, t14122: F, t14127: F, t14158: F, t14161: F, t14166: F, t14171: F, t1883: F, t3924: F, t4004: F, t4057: F, t5675: F, t5735: F, t5745: F, t5755: F, t5767: F, t820: F) -> F {
    let t14189 = t869 * t14188;
    let t14191 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t14189;
    let t14192 = t225 * t9990;
    let t14193 = t213 * t14192;
    let t14200 = -F::cast_from(0.65854491829355115987e0_f64) * t820 * t10130 * t1883 + t14158 + F::cast_from(0.11565819519348392139e-2_f64) * t14161 - F::cast_from(0.65854491829355115987e0_f64) * t5755 * t5735 * t3924 + F::cast_from(0.73171657588172351096e-2_f64) * t14166 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t5767 * t4057 + F::cast_from(0.13170898365871023197e1_f64) * t820 * t14171 * t4004 + F::cast_from(0.26341796731742046394e1_f64) * t5745 * t14122 * t5675 + F::cast_from(0.26341796731742046394e1_f64) * t5745 * t14127 * t5675 - F::cast_from(0.13170898365871023197e1_f64) * t5755 * t14122 * t1399 - F::cast_from(0.13170898365871023197e1_f64) * t5755 * t14127 * t1399 - F::cast_from(0.10975748638225852664e-1_f64) * t10062 - t14191 - F::cast_from(0.39512695097613069591e1_f64) * t14193 * t5735 * t13805 - F::cast_from(0.65854491829355115987e0_f64) * t5755 * t5735 * t4057;
    t14200
}
