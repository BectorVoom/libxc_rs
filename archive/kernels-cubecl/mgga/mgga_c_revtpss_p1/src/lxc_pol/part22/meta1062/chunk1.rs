//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3793/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3793<F: Float>(t17306: F, t488: F, t1269: F, t6564: F, t1210: F, t12641: F, t1277: F, t1295: F, t17973: F, t17974: F, t17975: F, t17992: F, t17995: F, t18042: F, t18062: F, t18103: F, t20722: F, t20741: F, t21394: F, t21408: F, t3561: F, t3572: F, t3585: F, t5225: F, t5231: F, t5245: F, t5423: F, t5497: F, t59464: F) -> F {
    let t72927 = t17306 * t488;
    let t72933 = t6564 * t1269;
    let t72956 = -F::cast_from(0.52683593463484092788e1_f64) * t72927 * t17975 - F::cast_from(0.52683593463484092788e1_f64) * t17973 * t17974 * t18042 - F::cast_from(0.13170898365871023197e1_f64) * t72933 * t1295 + F::cast_from(0.52683593463484092788e1_f64) * t12641 * t20722 + F::cast_from(0.26341796731742046394e1_f64) * t1210 * t1277 * t5245 * t5497 - F::cast_from(0.26341796731742046394e1_f64) * t3572 * t20741 - F::cast_from(0.26341796731742046394e1_f64) * t17995 * t18103 + F::cast_from(0.52683593463484092788e1_f64) * t59464 * t5231 + F::cast_from(0.26341796731742046394e1_f64) * t18062 * t5423 - F::cast_from(0.13170898365871023197e1_f64) * t21394 * t3585 + F::cast_from(0.26341796731742046394e1_f64) * t5225 * t17992 + F::cast_from(0.52683593463484092788e1_f64) * t3561 * t21408;
    t72956
}
