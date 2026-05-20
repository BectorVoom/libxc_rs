//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2597/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2597<F: Float>(t1211: F, t20721: F, t1294: F, t6587: F, t1277: F, t1210: F, t1215: F, t1295: F, t1775: F, t18037: F, t20697: F, t20700: F, t20704: F, t20710: F, t20714: F, t3561: F, t3567: F, t3572: F, t3732: F, t5225: F, t5237: F, t5251: F, t5417: F, t5429: F, t5498: F, t6580: F, t6745: F) -> (F, F, F) {
    let t20722 = t1211 * t20721;
    let t20727 = t6587 * t1294;
    let t20728 = t1277 * t20727;
    let t20735 = -F::cast_from(0.65854491829355115987e0_f64) * t3732 * t6745 - F::cast_from(0.65854491829355115987e0_f64) * t20697 * t1215 - F::cast_from(0.65854491829355115987e0_f64) * t20700 * t1295 + F::cast_from(0.13170898365871023197e1_f64) * t3567 * t20704 - F::cast_from(0.65854491829355115987e0_f64) * t3561 * t6745 + F::cast_from(0.65854491829355115987e0_f64) * t1210 * t20710 - F::cast_from(0.13170898365871023197e1_f64) * t3567 * t20714 - F::cast_from(0.13170898365871023197e1_f64) * t5417 * t5498 - F::cast_from(0.13170898365871023197e1_f64) * t18037 * t1775 + F::cast_from(0.26341796731742046394e1_f64) * t3567 * t20722 + F::cast_from(0.13170898365871023197e1_f64) * t5251 * t5237 + F::cast_from(0.65854491829355115987e0_f64) * t1210 * t20728 + F::cast_from(0.13170898365871023197e1_f64) * t3572 * t6580 + F::cast_from(0.26341796731742046394e1_f64) * t5225 * t5429;
    (t20722, t20728, t20735)
}
