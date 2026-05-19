//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 851/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk851<F: Float>(t225: F, t6041: F, t1579: F, t2770: F, t1559: F, t213: F, t234: F, t2776: F, t2780: F, t2796: F, t2810: F, t2811: F, t4497: F, t4501: F, t4520: F, t4524: F, t4526: F, t5978: F, t6017: F, t6022: F, t820: F, t879: F) -> (F, F, F, F) {
    let t6042 = t6041 * t225;
    let t6048 = t1579 * t1579;
    let t6049 = t2770 * t6048;
    let t6071 = t2776 - t2780 + F::cast_from(0.10975748638225852664e-1_f64) * t4497 - F::cast_from(0.10975748638225852664e-1_f64) * t4520 + t2796 - F::cast_from(0.19514881078765566038e-1_f64) * t4501 + F::cast_from(0.19514881078765566038e-1_f64) * t4524 - t2810 + F::cast_from(0.13170898365871023197e1_f64) * t820 * t2811 * t6022 - F::cast_from(0.13170898365871023197e1_f64) * t820 * t4526 * t1559 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t879 * t6017 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t879 * t5978 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t234 * t6041;
    (t6042, t6048, t6049, t6071)
}
