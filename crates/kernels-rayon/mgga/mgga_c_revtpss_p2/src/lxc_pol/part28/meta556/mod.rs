//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2009;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta556(t92975: f64, t243: f64, t7021: f64, t2732: f64, t64: f64, t9731: f64, t2710: f64, t826: f64, t10631: f64, t10886: f64, t7028: f64, t159: f64, t8779: f64, t218: f64, t816: f64, t10685: f64, t1946: f64, t10671: f64, t7033: f64, t25255: f64, t2689: f64, t10680: f64, t1945: f64, t807: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92976, t92979, t92986, t92989, t92991, t92993) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2009(t92975, t243, t7021, t2732, t64, t9731, t2710, t826, t10631, t10886, t7028, t159, t8779);
        let (t92996, t92998, t93000, t93001, t93004) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2010(t218, t816, t92993, t10685, t1946, t10671, t7033, t25255, t2689, t10680, t1945, t807);
    (t92976, t92979, t92986, t92989, t92991, t92993, t92996, t92998, t93000, t93001, t93004)
}
