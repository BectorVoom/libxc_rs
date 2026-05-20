//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3795/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3795<F: Float>(t1210: F, t1214: F, t12673: F, t1274: F, t1277: F, t1294: F, t17973: F, t17974: F, t17986: F, t17987: F, t17998: F, t18018: F, t18062: F, t18097: F, t20697: F, t20744: F, t21617: F, t21621: F, t3576: F, t3585: F, t3737: F, t45433: F, t5246: F, t56707: F, t6574: F, t6703: F, t6745: F) -> F {
    let t73020 = -F::cast_from(0.65854491829355115987e0_f64) * t21621 * t3585 + F::cast_from(0.26341796731742046394e1_f64) * t1274 * t3737 * t21617 * t1294 - F::cast_from(0.52683593463484092788e1_f64) * t56707 * t20744 - F::cast_from(0.26341796731742046394e1_f64) * t18062 * t5246 - F::cast_from(0.52683593463484092788e1_f64) * t17986 * t17987 * t18018 - F::cast_from(0.26341796731742046394e1_f64) * t18097 * t5246 - F::cast_from(0.26341796731742046394e1_f64) * t17973 * t17974 * t17998 - F::cast_from(0.65854491829355115987e0_f64) * t12673 * t6745 + F::cast_from(0.13170898365871023197e1_f64) * t20697 * t3576 + F::cast_from(0.13170898365871023197e1_f64) * t1210 * t1277 * t21617 * t1214 + F::cast_from(0.13170898365871023197e1_f64) * t12673 * t6703 + F::cast_from(0.26341796731742046394e1_f64) * t45433 * t6574;
    t73020
}
