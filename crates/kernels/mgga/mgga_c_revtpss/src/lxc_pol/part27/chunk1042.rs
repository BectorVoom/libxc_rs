//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1042/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1042<F: Float>(t1294: F, t7644: F, t7652: F, t1204: F, t2142: F, t1209: F, t26936: F, t1214: F, t7627: F, t7637: F, t1215: F, t1295: F, t2149: F, t2152: F, t26971: F, t26976: F, t26979: F, t26984: F, t26988: F, t26991: F, t26994: F, t26996: F, t26999: F, t27005: F, t27008: F, t27011: F, t3569: F, t3576: F, t3585: F, t3739: F, t7602: F, t7632: F, t7639: F, t7643: F, t7645: F, t7648: F, t7651: F, t7666: F) -> (F, F, F, F, F, F) {
    let t27015 = t7652 * t7644 * t1294;
    let t27020 = t1204 * t2142;
    let t27025 = t1209 * t26936;
    let t27028 = t7627 * t1214;
    let t27029 = t7637 * t27028;
    let t27032 = -0.26020884564615598386e1 * t7651 * t26971 + 0.13170898365871023197e1 * t7632 * t3739 + 0.13170898365871023197e1 * t26976 * t3569 + 0.17347256376410398924e1 * t26979 * t7645 - 0.8673628188205199462e0 * t26984 * t2152 + 0.8673628188205199462e0 * t7651 * t26988 - 0.4336814094102599731e0 * t26991 * t2152 + 0.34694512752820797848e1 * t26994 * t26996 - 0.13170898365871023197e1 * t26999 * t1215 - 0.65854491829355115987e0 * t7602 * t3585 - 0.4336814094102599731e0 * t2149 * t27005 - 0.13170898365871023197e1 * t27008 * t1295 - 0.13170898365871023197e1 * t27011 * t1215 - 0.34694512752820797848e1 * t7643 * t27015 - 0.8673628188205199462e0 * t7648 * t7666 - 0.13170898365871023197e1 * t27020 * t1295 + 0.13170898365871023197e1 * t7602 * t3576 - 0.17347256376410398924e1 * t27025 * t7639 + 0.17347256376410398924e1 * t7643 * t27029;
    (t27015, t27020, t27025, t27028, t27029, t27032)
}
