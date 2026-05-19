//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1169/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1169<F: Float>(t1215: F, t1295: F, t2149: F, t2152: F, t26971: F, t26976: F, t26979: F, t26984: F, t26988: F, t26991: F, t26994: F, t26996: F, t26999: F, t27005: F, t27008: F, t27011: F, t27015: F, t27020: F, t27025: F, t27029: F, t3569: F, t3576: F, t3585: F, t3739: F, t7602: F, t7632: F, t7639: F, t7643: F, t7645: F, t7648: F, t7651: F, t7666: F) -> F {
    let t27032 = -F::cast_from(0.26020884564615598386e1_f64) * t7651 * t26971 + F::cast_from(0.13170898365871023197e1_f64) * t7632 * t3739 + F::cast_from(0.13170898365871023197e1_f64) * t26976 * t3569 + F::cast_from(0.17347256376410398924e1_f64) * t26979 * t7645 - F::cast_from(0.8673628188205199462e0_f64) * t26984 * t2152 + F::cast_from(0.8673628188205199462e0_f64) * t7651 * t26988 - F::cast_from(0.4336814094102599731e0_f64) * t26991 * t2152 + F::cast_from(0.34694512752820797848e1_f64) * t26994 * t26996 - F::cast_from(0.13170898365871023197e1_f64) * t26999 * t1215 - F::cast_from(0.65854491829355115987e0_f64) * t7602 * t3585 - F::cast_from(0.4336814094102599731e0_f64) * t2149 * t27005 - F::cast_from(0.13170898365871023197e1_f64) * t27008 * t1295 - F::cast_from(0.13170898365871023197e1_f64) * t27011 * t1215 - F::cast_from(0.34694512752820797848e1_f64) * t7643 * t27015 - F::cast_from(0.8673628188205199462e0_f64) * t7648 * t7666 - F::cast_from(0.13170898365871023197e1_f64) * t27020 * t1295 + F::cast_from(0.13170898365871023197e1_f64) * t7602 * t3576 - F::cast_from(0.17347256376410398924e1_f64) * t27025 * t7639 + F::cast_from(0.17347256376410398924e1_f64) * t7643 * t27029;
    t27032
}
