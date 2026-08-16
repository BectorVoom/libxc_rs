//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3794/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3794<F: Float>(t21342: F, t460: F, t1204: F, t12633: F, t12641: F, t1274: F, t1295: F, t13182: F, t1775: F, t18037: F, t18062: F, t18109: F, t1829: F, t20704: F, t20714: F, t20741: F, t20756: F, t21344: F, t3552: F, t3556: F, t3738: F, t3739: F, t5237: F, t5417: F, t5423: F, t56396: F, t56575: F, t6697: F, t6744: F) -> F {
    let t72959 = t460 * t21342;
    let t72986 = F::cast_from(0.26341796731742046394e1_f64) * t18037 * t5423 - F::cast_from(0.13170898365871023197e1_f64) * t72959 * t1295 + F::cast_from(0.13170898365871023197e1_f64) * t1204 * t21344 - F::cast_from(0.26341796731742046394e1_f64) * t3556 * t20741 - F::cast_from(0.39512695097613069591e1_f64) * t1274 * t13182 * t6744 * t3738 + F::cast_from(0.52683593463484092788e1_f64) * t5417 * t18109 + F::cast_from(0.26341796731742046394e1_f64) * t12641 * t20704 - F::cast_from(0.26341796731742046394e1_f64) * t12633 * t20714 + F::cast_from(0.26341796731742046394e1_f64) * t18062 * t5237 + F::cast_from(0.65854491829355115987e0_f64) * t3552 * t6697 + F::cast_from(0.26341796731742046394e1_f64) * t20756 * t3739 - F::cast_from(0.13170898365871023197e1_f64) * t56396 * t1775 - F::cast_from(0.26341796731742046394e1_f64) * t56575 * t1829;
    t72986
}
