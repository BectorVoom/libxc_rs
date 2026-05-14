//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1408/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1408<F: Float>(t1245: F, t3199: F, t2393: F, t3903: F, t10092: F, t10311: F, t10319: F, t10320: F, t10334: F, t133: F, t19251: F, t19283: F, t19297: F, t2363: F, t2448: F, t26940: F, t27187: F, t28016: F, t28400: F, t2970: F, t3207: F, t3259: F, t3260: F, t3914: F, t3923: F, t397: F, t6514: F, t6523: F, t7832: F, t8508: F, t8519: F, t8542: F, t919: F, t943: F, t945: F) -> (F, F) {
    let t28424 = t1245 * t3199;
    let t28435 = t2393 * t3903;
    let t28438 = -0.39512695097613069591e1 * t10319 * t19283 + 0.65854491829355115987e0 * t397 * t28400 - 0.79025390195226139182e1 * t6523 * t26940 * t10320 + 0.65854491829355115987e0 * t943 * t28016 * t133 * t945 + 0.79025390195226139182e1 * t6514 * t26940 * t10311 + 0.26341796731742046394e1 * t8542 * t7832 * t3207 * t3199 - 0.15805078039045227836e2 * t8519 * t7832 * t10092 * t919 + 0.39512695097613069591e1 * t6514 * t10334 * t8508 + 0.52683593463484092788e1 * t2363 * t28424 * t3260 + 0.13170898365871023197e1 * t19251 * t3914 + 0.26341796731742046394e1 * t3259 * t2970 * t27187 - 0.65854491829355115987e0 * t19297 * t3923 - 0.65854491829355115987e0 * t28435 * t2448;
    (t28424, t28438)
}
