//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3898/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3898(t1437: f64, t2482: f64, t4104: f64, t6861: f64, t1432: f64, t22307: f64, t686: f64, t72: f64, t1385: f64, t1399: f64, t46392: f64, t46398: f64, t46401: f64, t46412: f64, t47957: f64, t73937: f64, t74167: f64, t74862: f64, t74866: f64, t74873: f64, t820: f64) -> f64 {
    let t74880 = t2482 * t1437 * t6861 * t4104;
    let t74884 = t1432 * t22307 * t72 * t686;
    let t74886 = t1385 * t22307;
    let t74890 = -0.39274398764404314548e-3_f64 * t46392 + 0.73171657588172351096e-2_f64 * t46398 - 0.65049603595885220126e-3_f64 * t46401 - 0.39029762157531132075e-1_f64 * t74862 + t46412 + 0.39029762157531132075e-1_f64 * t74866 - 0.19514881078765566038e-1_f64 * t47957 - 0.65854491829355115987e0_f64 * t820 * t1437 * t74167 - 0.13009920719177044025e-1_f64 * t74873 - 0.13170898365871023197e1_f64 * t820 * t1437 * t73937 - 0.19514881078765566038e-1_f64 * t74880 + 0.19514881078765566038e-1_f64 * t74884 - 0.13170898365871023197e1_f64 * t820 * t74886 * t1399;
    t74890
}
