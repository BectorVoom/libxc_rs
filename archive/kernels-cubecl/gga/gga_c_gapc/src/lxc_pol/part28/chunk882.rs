//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 882/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk882<F: Float>(t1081: F, t2804: F, t10000: F, t10003: F, t10006: F, t10010: F, t10014: F, t10016: F, t10019: F, t9986: F, t9991: F, t9993: F, t9995: F, t9997: F) -> (F, F) {
    let t10021 = t1081 * t2804;
    let t10023 = F::cast_from(0.14758978949652777778e-5_f64) * t9986 + F::cast_from(0.42205124476153752644e-7_f64) * t9991 - F::cast_from(0.1374296967252737644e-5_f64) * t9993 + F::cast_from(0.1081184847736214213e-1_f64) * t9995 + F::cast_from(0.2813674965076916843e-7_f64) * t9997 + F::cast_from(0.2813674965076916843e-7_f64) * t10000 - F::cast_from(0.13900948042322754167e-2_f64) * t10003 - F::cast_from(0.39192950730437765221e-2_f64) * t10006 + F::cast_from(0.50680539737635041234e-4_f64) * t10010 + F::cast_from(0.7324140771837707598e-5_f64) * t10014 + F::cast_from(0.75883739738679928911e-6_f64) * t10016 + F::cast_from(0.27801896084645508334e-2_f64) * t10019 - F::cast_from(0.6956508833112845217e-4_f64) * t10021;
    (t10021, t10023)
}
