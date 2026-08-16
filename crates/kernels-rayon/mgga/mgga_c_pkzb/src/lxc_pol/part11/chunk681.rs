//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 681/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk681(t3928: f64, t942: f64, t1246: f64, t1256: f64, t3904: f64, t3910: f64, t411: f64, t415: f64, t1259: f64, t135: f64, t2464: f64, t273: f64, t3736: f64, t3738: f64, t3742: f64, t3768: f64, t3771: f64, t3827: f64, t3829: f64, t3831: f64, t3835: f64, t3839: f64, t3843: f64, t957: f64) -> (f64, f64, f64, f64) {
    let t3929 = t942 * t3928;
    let t3932 = 0.65854491829355115987e0_f64 * t3904 * t415 - 0.13170898365871023197e1_f64 * t1246 * t1256 + 0.13170898365871023197e1_f64 * t411 * t3910 - 0.65854491829355115987e0_f64 * t411 * t3929;
    let t3936 = t1259 * t1259;
    let t3940 = -t135 * t2464 * t273 * t3936 + t135 * t273 * t3932 * t957 - t3736 + t3738 - t3742 + t3768 + t3771 + t3827 + t3829 - t3831 + t3835 - t3839 - t3843;
    (t3929, t3932, t3936, t3940)
}
