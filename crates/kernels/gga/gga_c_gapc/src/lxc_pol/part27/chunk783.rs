//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 783/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk783<F: Float>(t10012: F, t10013: F, t2664: F, t9501: F, t2316: F, t2636: F, t3378: F, t1081: F, t2804: F, t10000: F, t10003: F, t10006: F, t10010: F, t9986: F, t9991: F, t9993: F, t9995: F, t9997: F) -> (F,) {
    let t10014 = t10012 * t10013;
    let t10016 = t9501 * t2664;
    let t10018 = t2636 * t2316;
    let t10019 = t3378 * t10018;
    let t10021 = t1081 * t2804;
    let t10023 = 0.14758978949652777778e-5 * t9986 + 0.42205124476153752644e-7 * t9991 - 0.1374296967252737644e-5 * t9993 + 0.1081184847736214213e-1 * t9995 + 0.2813674965076916843e-7 * t9997 + 0.2813674965076916843e-7 * t10000 - 0.13900948042322754167e-2 * t10003 - 0.39192950730437765221e-2 * t10006 + 0.50680539737635041234e-4 * t10010 + 0.7324140771837707598e-5 * t10014 + 0.75883739738679928911e-6 * t10016 + 0.27801896084645508334e-2 * t10019 - 0.6956508833112845217e-4 * t10021;
    (t10023,)
}
